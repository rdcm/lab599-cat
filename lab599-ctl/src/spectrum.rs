use std::sync::{Arc, Mutex};

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    SampleFormat, StreamConfig,
};
use rustfft::{num_complex::Complex, FftPlanner};

pub const FFT_SIZE: usize = 2048;

pub type SpectrumBins = Arc<Mutex<Vec<f32>>>;

pub struct IqCapture {
    _stream: cpal::Stream,
    pub sample_rate: u32,
    pub bins: SpectrumBins,
    pub is_stereo: Arc<Mutex<bool>>,
}

pub fn start_iq_capture(device: cpal::Device, sample_rate: u32) -> Result<IqCapture> {
    let bins: SpectrumBins = Arc::new(Mutex::new(vec![-100.0f32; FFT_SIZE]));

    let cfg_range = device
        .supported_input_configs()?
        .filter(|c| {
            (c.channels() == 2 || c.channels() == 1)
                && c.min_sample_rate() <= sample_rate
                && c.max_sample_rate() >= sample_rate
        })
        .min_by_key(|c| c.channels())
        .ok_or_else(|| anyhow::anyhow!("audio device has no config at {sample_rate} Hz"))?;

    let channels = cfg_range.channels() as usize;
    let is_stereo: Arc<Mutex<bool>> = Arc::new(Mutex::new(channels >= 2));
    let fmt = cfg_range.sample_format();
    let config: StreamConfig = cfg_range.with_sample_rate(sample_rate).into();

    let stream = build_stream(
        &device,
        &config,
        fmt,
        channels,
        bins.clone(),
        is_stereo.clone(),
    )?;
    stream.play()?;

    Ok(IqCapture {
        _stream: stream,
        sample_rate,
        bins,
        is_stereo,
    })
}

fn build_stream(
    device: &cpal::Device,
    config: &StreamConfig,
    fmt: SampleFormat,
    channels: usize,
    bins: SpectrumBins,
    is_stereo: Arc<Mutex<bool>>,
) -> Result<cpal::Stream> {
    let err_fn = |e: cpal::StreamError| eprintln!("IQ stream error: {e}");

    let stream = match fmt {
        SampleFormat::F32 => {
            let mut proc = IqProcessor::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[f32], _| proc.push(data),
                err_fn,
                None,
            )?
        }
        SampleFormat::I16 => {
            let mut proc = IqProcessor::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[i16], _| {
                    let f: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    proc.push(&f);
                },
                err_fn,
                None,
            )?
        }
        SampleFormat::I32 => {
            let mut proc = IqProcessor::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[i32], _| {
                    let f: Vec<f32> = data.iter().map(|&s| s as f32 / i32::MAX as f32).collect();
                    proc.push(&f);
                },
                err_fn,
                None,
            )?
        }
        SampleFormat::F64 => {
            let mut proc = IqProcessor::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[f64], _| {
                    let f: Vec<f32> = data.iter().map(|&s| s as f32).collect();
                    proc.push(&f);
                },
                err_fn,
                None,
            )?
        }
        other => anyhow::bail!("unsupported sample format: {other:?}"),
    };

    Ok(stream)
}

struct IqProcessor {
    buf: Vec<f32>,
    channels: usize,
    // Tracks whether both channels carry independent signal.
    // Set to false when one channel is silent (PipeWire mono-in-stereo wrapper).
    real_stereo: bool,
    fft: std::sync::Arc<dyn rustfft::Fft<f32>>,
    hann: Vec<f32>,
    complex: Vec<Complex<f32>>,
    bins: SpectrumBins,
    pub is_stereo_flag: Arc<Mutex<bool>>,
    // IIR DC blocker state (one per channel)
    dc_i: f32,
    dc_q: f32,
    // Frequency-domain IQ imbalance coefficient (complex, Gram-Schmidt in freq domain)
    iq_mu: Complex<f32>,
}

impl IqProcessor {
    fn new(bins: SpectrumBins, channels: usize, is_stereo_flag: Arc<Mutex<bool>>) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(FFT_SIZE);
        let hann = (0..FFT_SIZE)
            .map(|i| {
                0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (FFT_SIZE - 1) as f32).cos())
            })
            .collect();
        Self {
            buf: Vec::with_capacity(FFT_SIZE * 4),
            channels,
            real_stereo: channels >= 2,
            fft,
            hann,
            complex: vec![Complex::default(); FFT_SIZE],
            bins,
            is_stereo_flag,
            dc_i: 0.0,
            dc_q: 0.0,
            iq_mu: Complex::new(0.0, 0.0),
        }
    }

    fn push(&mut self, data: &[f32]) {
        // Detect PipeWire mono-wrapped-as-stereo: one channel silent.
        if self.channels >= 2 {
            let rms_i: f32 =
                data.iter().step_by(2).map(|x| x * x).sum::<f32>() / (data.len() / 2) as f32;
            let rms_q: f32 = data.iter().skip(1).step_by(2).map(|x| x * x).sum::<f32>()
                / (data.len() / 2) as f32;
            // One channel is at least 20 dB quieter → not real IQ
            let was = self.real_stereo;
            self.real_stereo =
                rms_i > 1e-10 && rms_q > 1e-10 && (rms_i / rms_q).max(rms_q / rms_i) < 100.0;
            if was != self.real_stereo {
                if let Ok(mut flag) = self.is_stereo_flag.lock() {
                    *flag = self.real_stereo;
                }
            }
        }

        self.buf.extend_from_slice(data);
        let hop = FFT_SIZE / 2 * self.channels;
        let use_stereo = self.channels >= 2 && self.real_stereo;

        while self.buf.len() >= FFT_SIZE * self.channels {
            if use_stereo {
                // IIR DC blocker: α=0.9995 → time constant ~83 ms at 48 kHz.
                // ch[0]=L=AUX Q (pin4), ch[1]=R=AUX I (pin5) per TX-500 cable wiring.
                const DC_ALPHA: f32 = 0.9995;
                for (i, ch) in self.buf.chunks(2).take(FFT_SIZE).enumerate() {
                    let raw_i = ch.get(1).copied().unwrap_or(0.0);
                    let raw_q = ch[0];
                    self.dc_i = DC_ALPHA * self.dc_i + (1.0 - DC_ALPHA) * raw_i;
                    self.dc_q = DC_ALPHA * self.dc_q + (1.0 - DC_ALPHA) * raw_q;
                    self.complex[i] = Complex {
                        re: (raw_i - self.dc_i) * self.hann[i],
                        im: (raw_q - self.dc_q) * self.hann[i],
                    };
                }
            } else {
                // Mono or pseudo-stereo: use only the live channel
                let step = self.channels;
                for (i, s) in self.buf.iter().step_by(step).take(FFT_SIZE).enumerate() {
                    self.complex[i] = Complex {
                        re: s * self.hann[i],
                        im: 0.0,
                    };
                }
            }
            self.buf.drain(..hop);

            self.fft.process(&mut self.complex);

            if use_stereo {
                // Frequency-domain IQ imbalance correction.
                // Model: Y[k] = X[k] + μ·conj(X[N-k])  (image at mirror bin due to I/Q mismatch).
                // Estimate μ from spectral cross-correlation, then subtract the image.
                // Weighted μ estimator: symmetric bin pairs (image candidates) get high weight;
                // strongly asymmetric pairs (real signals) get near-zero weight to avoid bias.
                const MU_ADAPT: f32 = 0.01;
                let mut cross = Complex::<f32>::new(0.0, 0.0);
                let mut power = 0.0f32;
                for k in 1..FFT_SIZE / 2 {
                    let yk = self.complex[k];
                    let ynk = self.complex[FFT_SIZE - k];
                    let pk = yk.norm_sqr();
                    let pnk = ynk.norm_sqr();
                    let sym_weight = (pk.min(pnk) / (pk.max(pnk) + 1e-20)).powi(2);
                    cross += (ynk * yk.conj()) * sym_weight;
                    power += pk * sym_weight;
                }
                if power > 1e-10 {
                    let mu_inst = cross / power;
                    self.iq_mu = self.iq_mu * (1.0 - MU_ADAPT) + mu_inst * MU_ADAPT;
                }
                let scale = 1.0 / (1.0 - self.iq_mu.norm_sqr()).max(0.1);
                for k in 1..FFT_SIZE / 2 {
                    let yk = self.complex[k];
                    let ynk = self.complex[FFT_SIZE - k];
                    self.complex[k] = (yk - self.iq_mu * ynk.conj()) * scale;
                    self.complex[FFT_SIZE - k] = (ynk - self.iq_mu * yk.conj()) * scale;
                }
            }

            const ALPHA: f32 = 0.5;
            if let Ok(mut s) = self.bins.lock() {
                if use_stereo {
                    for i in 0..FFT_SIZE {
                        let shifted = (i + FFT_SIZE / 2) % FFT_SIZE;
                        let mag = self.complex[i].norm();
                        let db = 20.0 * (mag / FFT_SIZE as f32 + 1e-10).log10();
                        s[shifted] = ALPHA * db + (1.0 - ALPHA) * s[shifted];
                    }
                    // Null the LO leakage spike at DC (center ±8 bins = ±188 Hz at 48 kHz)
                    const DC_NULL: usize = 8;
                    let center = FFT_SIZE / 2;
                    for v in s
                        [center.saturating_sub(DC_NULL)..=(center + DC_NULL).min(FFT_SIZE - 1)]
                        .iter_mut()
                    {
                        *v = -100.0;
                    }
                } else {
                    for i in 0..FFT_SIZE / 2 {
                        let mag = self.complex[i].norm();
                        let db = 20.0 * (mag / FFT_SIZE as f32 + 1e-10).log10();
                        s[i] = ALPHA * db + (1.0 - ALPHA) * s[i];
                    }
                    for v in s[FFT_SIZE / 2..].iter_mut() {
                        *v = -100.0;
                    }
                }
            }
        }
    }
}
