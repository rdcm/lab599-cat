use std::collections::VecDeque;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};

use crate::hardware::spectrum::SpectrumBins;
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};

type Clients = Arc<Mutex<Vec<SyncSender<Vec<f32>>>>>;

#[derive(Clone)]
pub enum AudioMode {
    Loopback,
    Remote,
}

pub struct Audio {
    _mode: Option<AudioMode>,
    _audio_in: Option<cpal::Stream>,
    _audio_out: Option<cpal::Stream>,
    _iq_in: Option<cpal::Stream>,
    iq_sample_rate: u32,
    bins: Option<SpectrumBins>,
    is_stereo: Option<Arc<Mutex<bool>>>,
    errors: Arc<Mutex<Vec<String>>>,
}

impl Audio {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mode: Option<AudioMode>,
        audio_in: Option<cpal::Stream>,
        audio_out: Option<cpal::Stream>,
        iq_in: Option<cpal::Stream>,
        iq_sample_rate: u32,
        bins: Option<SpectrumBins>,
        is_stereo: Option<Arc<Mutex<bool>>>,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Self {
        Self {
            _mode: mode,
            _audio_in: audio_in,
            _audio_out: audio_out,
            _iq_in: iq_in,
            iq_sample_rate,
            bins,
            is_stereo,
            errors,
        }
    }

    pub fn errors(&self) -> &Arc<Mutex<Vec<String>>> {
        &self.errors
    }

    pub fn bins(&self) -> Option<&SpectrumBins> {
        self.bins.as_ref()
    }

    pub fn is_stereo(&self) -> Option<&Arc<Mutex<bool>>> {
        self.is_stereo.as_ref()
    }

    pub fn iq_sample_rate(&self) -> u32 {
        self.iq_sample_rate
    }

    pub fn list_devices() {
        let host = cpal::default_host();
        println!("Available audio input devices:");
        if let Ok(devices) = host.input_devices() {
            for d in devices {
                if let Ok(name) = d.description() {
                    println!("  {name}");
                }
            }
        }
    }

    fn resolve_device(name_pattern: Option<&str>) -> Option<cpal::Device> {
        let host = cpal::default_host();
        match name_pattern {
            Some(pattern) => host.input_devices().ok()?.find(|d| {
                d.description()
                    .map(|n| n.name().to_lowercase().contains(&pattern.to_lowercase()))
                    .unwrap_or(false)
            }),
            None => host.default_input_device(),
        }
    }

    pub(super) fn build_transport(
        device_name: Option<&str>,
        rx_socket: Option<&Path>,
        mode: &AudioMode,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<(cpal::Stream, Option<cpal::Stream>)> {
        let label = device_name.unwrap_or("default");
        let device = Self::resolve_device(device_name)
            .ok_or_else(|| anyhow::anyhow!("audio device not found: {label}"))?;
        let clients: Clients = Arc::new(Mutex::new(Vec::new()));

        let (input, output, sample_rate) = match mode {
            AudioMode::Loopback => {
                let (i, o, sr) = Self::build_loopback(device, clients.clone(), errors)?;
                (i, Some(o), sr)
            }
            AudioMode::Remote => {
                let (i, sr) = Self::build_input_only(device, clients.clone(), errors)?;
                (i, None, sr)
            }
        };

        if let Some(socket) = rx_socket {
            match Self::start_rx_socket(socket, clients) {
                Ok(()) => eprintln!(
                    "RX socket: {0}\n  → nc -U {0} | aplay -f FLOAT_LE -r {1} -c 1",
                    socket.display(),
                    sample_rate,
                ),
                Err(e) => eprintln!("RX socket error: {e}"),
            }
        }

        Ok((input, output))
    }

    fn build_loopback(
        input_device: cpal::Device,
        clients: Clients,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<(cpal::Stream, cpal::Stream, u32)> {
        let host = cpal::default_host();
        let output_device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("no output audio device found"))?;

        let in_cfg = input_device.default_input_config()?;
        let out_cfg = output_device.default_output_config()?;

        let sample_rate = in_cfg.sample_rate();
        let in_channels = in_cfg.channels() as usize;

        let buf: Arc<Mutex<VecDeque<f32>>> = Arc::new(Mutex::new(VecDeque::new()));
        let buf_in = buf.clone();
        let buf_out = buf;

        let make_err = {
            let errors = errors.clone();
            move || {
                let errs = errors.clone();
                move |e: cpal::StreamError| {
                    if let Ok(mut q) = errs.lock() {
                        q.push(e.to_string());
                    }
                }
            }
        };

        let input_stream = match in_cfg.sample_format() {
            SampleFormat::F32 => input_device.build_input_stream(
                &in_cfg.into(),
                move |data: &[f32], _| {
                    let mono = Self::downmix_f32(data, in_channels);
                    Self::push_to_buf(&mono, &buf_in);
                    Self::broadcast(&mono, &clients);
                },
                make_err(),
                None,
            )?,
            SampleFormat::I16 => input_device.build_input_stream(
                &in_cfg.into(),
                move |data: &[i16], _| {
                    let mono = Self::downmix_i16(data, in_channels);
                    Self::push_to_buf(&mono, &buf_in);
                    Self::broadcast(&mono, &clients);
                },
                make_err(),
                None,
            )?,
            SampleFormat::U16 => input_device.build_input_stream(
                &in_cfg.into(),
                move |data: &[u16], _| {
                    let mono = Self::downmix_u16(data, in_channels);
                    Self::push_to_buf(&mono, &buf_in);
                    Self::broadcast(&mono, &clients);
                },
                make_err(),
                None,
            )?,
            _ => anyhow::bail!("unsupported input sample format"),
        };

        let out_channels = out_cfg.channels() as usize;

        let output_stream = match out_cfg.sample_format() {
            SampleFormat::F32 => output_device.build_output_stream(
                &out_cfg.into(),
                move |data: &mut [f32], _| Self::drain_to_output(data, out_channels, &buf_out),
                make_err(),
                None,
            )?,
            _ => anyhow::bail!("unsupported output sample format"),
        };

        input_stream.play()?;
        output_stream.play()?;

        Ok((input_stream, output_stream, sample_rate))
    }

    fn build_input_only(
        input_device: cpal::Device,
        clients: Clients,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<(cpal::Stream, u32)> {
        let in_cfg = input_device.default_input_config()?;
        let sample_rate = in_cfg.sample_rate();
        let in_channels = in_cfg.channels() as usize;

        let make_err = {
            let errors = errors.clone();
            move || {
                let errs = errors.clone();
                move |e: cpal::StreamError| {
                    if let Ok(mut q) = errs.lock() {
                        q.push(e.to_string());
                    }
                }
            }
        };

        let input_stream = match in_cfg.sample_format() {
            SampleFormat::F32 => input_device.build_input_stream(
                &in_cfg.into(),
                move |data: &[f32], _| {
                    Self::broadcast(&Self::downmix_f32(data, in_channels), &clients)
                },
                make_err(),
                None,
            )?,
            SampleFormat::I16 => input_device.build_input_stream(
                &in_cfg.into(),
                move |data: &[i16], _| {
                    Self::broadcast(&Self::downmix_i16(data, in_channels), &clients)
                },
                make_err(),
                None,
            )?,
            SampleFormat::U16 => input_device.build_input_stream(
                &in_cfg.into(),
                move |data: &[u16], _| {
                    Self::broadcast(&Self::downmix_u16(data, in_channels), &clients)
                },
                make_err(),
                None,
            )?,
            _ => anyhow::bail!("unsupported input sample format"),
        };

        input_stream.play()?;
        Ok((input_stream, sample_rate))
    }

    fn start_rx_socket(path: &Path, clients: Clients) -> Result<()> {
        let _ = std::fs::remove_file(path);
        let listener = UnixListener::bind(path)?;

        std::thread::spawn(move || {
            for mut socket in listener.incoming().flatten() {
                let (tx, rx) = mpsc::sync_channel::<Vec<f32>>(32);
                clients.lock().unwrap().push(tx);
                std::thread::spawn(move || {
                    for chunk in rx {
                        for sample in chunk {
                            if socket.write_all(&sample.to_le_bytes()).is_err() {
                                return;
                            }
                        }
                    }
                });
            }
        });

        Ok(())
    }

    fn broadcast(mono: &[f32], clients: &Mutex<Vec<SyncSender<Vec<f32>>>>) {
        let mut list = match clients.lock() {
            Ok(l) => l,
            Err(_) => return,
        };
        if list.is_empty() {
            return;
        }
        let chunk = mono.to_vec();
        list.retain(|tx| match tx.try_send(chunk.clone()) {
            Ok(_) | Err(TrySendError::Full(_)) => true,
            Err(TrySendError::Disconnected(_)) => false,
        });
    }

    fn push_to_buf(mono: &[f32], buf: &Mutex<VecDeque<f32>>) {
        if let Ok(mut b) = buf.lock() {
            b.extend(mono);
            let len = b.len();
            if len > 96_000 {
                b.drain(..len - 96_000);
            }
        }
    }

    fn downmix_f32(data: &[f32], channels: usize) -> Vec<f32> {
        data.chunks(channels)
            .map(|ch| ch.iter().sum::<f32>() / ch.len() as f32)
            .collect()
    }

    fn downmix_i16(data: &[i16], channels: usize) -> Vec<f32> {
        data.chunks(channels)
            .map(|ch| ch.iter().map(|&s| s as f32 / i16::MAX as f32).sum::<f32>() / ch.len() as f32)
            .collect()
    }

    fn downmix_u16(data: &[u16], channels: usize) -> Vec<f32> {
        data.chunks(channels)
            .map(|ch| {
                ch.iter()
                    .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                    .sum::<f32>()
                    / ch.len() as f32
            })
            .collect()
    }

    fn drain_to_output(data: &mut [f32], channels: usize, buf: &Mutex<VecDeque<f32>>) {
        if let Ok(mut b) = buf.lock() {
            for frame in data.chunks_mut(channels) {
                let sample = b.pop_front().unwrap_or(0.0);
                for s in frame.iter_mut() {
                    *s = sample;
                }
            }
        }
    }
}
