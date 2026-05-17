use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, StreamConfig,
};
use rustfft::{num_complex::Complex, FftPlanner};

pub const FFT_SIZE: usize = 2048;
pub type Bins = Arc<Mutex<Vec<f32>>>;

pub struct Spectrum {
    _stream: Option<cpal::Stream>,
    bins: Option<Bins>,
    sample_rate: u32,
    is_stereo: Option<Arc<Mutex<bool>>>,
    device_name: Option<String>,
}

impl Spectrum {
    pub fn new() -> Self {
        Self {
            _stream: None,
            bins: None,
            sample_rate: 0,
            is_stereo: None,
            device_name: None,
        }
    }

    pub fn start(
        &mut self,
        device: &str,
        rate: u32,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<()> {
        let (stream, bins, is_stereo, name) = crate::app_utils::capture_stderr(
            || open_stream(device, rate, errors.clone()),
            &errors,
        )?;
        self._stream = Some(stream);
        self.bins = Some(bins);
        self.is_stereo = Some(is_stereo);
        self.sample_rate = rate;
        self.device_name = Some(name);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self._stream.take() {
            struct SendStream {
                _s: cpal::Stream,
            }
            unsafe impl Send for SendStream {}
            std::thread::spawn(move || drop(SendStream { _s: stream }));
        }
        self.bins = None;
        self.is_stereo = None;
        self.sample_rate = 0;
        self.device_name = None;
    }

    pub fn is_active(&self) -> bool {
        self.bins.is_some()
    }

    pub fn bins(&self) -> Option<Bins> {
        self.bins.clone()
    }

    pub fn is_stereo(&self) -> Option<Arc<Mutex<bool>>> {
        self.is_stereo.clone()
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn device_name(&self) -> Option<&str> {
        self.device_name.as_deref()
    }
}

fn open_stream(
    name: &str,
    sample_rate: u32,
    errors: Arc<Mutex<Vec<String>>>,
) -> Result<(cpal::Stream, Bins, Arc<Mutex<bool>>, String)> {
    let host = cpal::default_host();
    let candidates: Vec<cpal::Device> = host
        .input_devices()
        .map(|devs| {
            devs.filter(|d| {
                let id = d.id().map(|i| i.to_string()).unwrap_or_default();
                !id.to_lowercase().contains("dsnoop")
                    && d.description()
                        .map(|n| n.name().to_lowercase() == name.to_lowercase())
                        .unwrap_or(false)
            })
            .collect()
        })
        .unwrap_or_default();

    if candidates.is_empty() {
        return Err(anyhow::anyhow!("device not found: {name}"));
    }
    let mut last_err = anyhow::anyhow!("no device usable at {sample_rate} Hz");
    for dev in candidates {
        let desc = dev
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_| name.to_string());
        match build_stream(dev, sample_rate, errors.clone()) {
            Ok((stream, bins, is_stereo)) => return Ok((stream, bins, is_stereo, desc)),
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

fn build_stream(
    device: cpal::Device,
    sample_rate: u32,
    errors: Arc<Mutex<Vec<String>>>,
) -> Result<(cpal::Stream, Bins, Arc<Mutex<bool>>)> {
    let bins: Bins = Arc::new(Mutex::new(vec![-100.0f32; FFT_SIZE]));

    let cfg_range = device
        .supported_input_configs()?
        .filter(|c| {
            (c.channels() == 2 || c.channels() == 1)
                && c.min_sample_rate() <= sample_rate
                && c.max_sample_rate() >= sample_rate
        })
        .max_by_key(|c| c.channels())
        .ok_or_else(|| anyhow::anyhow!("no supported config at {sample_rate} Hz"))?;

    let channels = cfg_range.channels() as usize;
    let is_stereo: Arc<Mutex<bool>> = Arc::new(Mutex::new(channels >= 2));
    let fmt = cfg_range.sample_format();
    let config: StreamConfig = cfg_range.with_sample_rate(sample_rate).into();

    let stream = build_callback(
        &device,
        &config,
        fmt,
        channels,
        bins.clone(),
        is_stereo.clone(),
        errors,
    )?;
    stream.play()?;

    Ok((stream, bins, is_stereo))
}

fn build_callback(
    device: &cpal::Device,
    config: &StreamConfig,
    fmt: SampleFormat,
    channels: usize,
    bins: Bins,
    is_stereo: Arc<Mutex<bool>>,
    errors: Arc<Mutex<Vec<String>>>,
) -> Result<cpal::Stream> {
    let err_fn = {
        let mut last: Option<(String, Instant)> = None;
        move |e: cpal::StreamError| {
            let msg = e.to_string();
            let now = Instant::now();
            let stale = last
                .as_ref()
                .map(|(prev, t)| prev != &msg || t.elapsed() >= Duration::from_secs(5))
                .unwrap_or(true);
            if stale {
                last = Some((msg.clone(), now));
                if let Ok(mut q) = errors.lock() {
                    q.push(msg);
                }
            }
        }
    };

    let stream = match fmt {
        SampleFormat::F32 => {
            let mut sink = FftSink::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[f32], _| sink.push(data),
                err_fn,
                None,
            )?
        }
        SampleFormat::I16 => {
            let mut sink = FftSink::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[i16], _| {
                    let f: Vec<f32> = data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    sink.push(&f);
                },
                err_fn,
                None,
            )?
        }
        SampleFormat::I32 => {
            let mut sink = FftSink::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[i32], _| {
                    let f: Vec<f32> = data.iter().map(|&s| s as f32 / i32::MAX as f32).collect();
                    sink.push(&f);
                },
                err_fn,
                None,
            )?
        }
        SampleFormat::F64 => {
            let mut sink = FftSink::new(bins, channels, is_stereo);
            device.build_input_stream(
                config,
                move |data: &[f64], _| {
                    let f: Vec<f32> = data.iter().map(|&s| s as f32).collect();
                    sink.push(&f);
                },
                err_fn,
                None,
            )?
        }
        other => anyhow::bail!("unsupported sample format: {other:?}"),
    };

    Ok(stream)
}

struct FftSink {
    buf: Vec<f32>,
    channels: usize,
    real_stereo: bool,
    fft: Arc<dyn rustfft::Fft<f32>>,
    hann: Vec<f32>,
    complex: Vec<Complex<f32>>,
    bins: Bins,
    is_stereo_flag: Arc<Mutex<bool>>,
    dc_i: f32,
    dc_q: f32,
    iq_mu: Complex<f32>,
}

impl FftSink {
    fn new(bins: Bins, channels: usize, is_stereo_flag: Arc<Mutex<bool>>) -> Self {
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
        if self.channels >= 2 {
            let rms_i: f32 =
                data.iter().step_by(2).map(|x| x * x).sum::<f32>() / (data.len() / 2) as f32;
            let rms_q: f32 = data.iter().skip(1).step_by(2).map(|x| x * x).sum::<f32>()
                / (data.len() / 2) as f32;
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
                const DC_ALPHA: f32 = 0.995;
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
