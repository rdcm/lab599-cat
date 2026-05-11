use std::collections::VecDeque;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};

type Clients = Arc<Mutex<Vec<SyncSender<Vec<f32>>>>>;

pub struct Audio {
    _input: cpal::Stream,
    _output: cpal::Stream,
}

impl Audio {
    pub fn new(device_name: &str, rx_socket: &Path) -> Option<Self> {
        let device = find_audio_device(device_name)?;
        let loopback = start_audio(device).ok()?;
        let sample_rate = loopback.sample_rate;
        match start_rx_socket(rx_socket, loopback.clients) {
            Ok(()) => eprintln!(
                "RX socket: {0}\n  → nc -U {0} | aplay -f FLOAT_LE -r {1} -c 1",
                rx_socket.display(),
                sample_rate,
            ),
            Err(e) => eprintln!("RX socket error: {e}"),
        }
        Some(Self {
            _input: loopback._input,
            _output: loopback._output,
        })
    }
}

struct AudioLoopback {
    _input: cpal::Stream,
    _output: cpal::Stream,
    clients: Clients,
    sample_rate: u32,
}

pub fn list_audio_devices() {
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

fn find_audio_device(name_pattern: &str) -> Option<cpal::Device> {
    let host = cpal::default_host();
    host.input_devices().ok()?.find(|d| {
        d.description()
            .map(|n| {
                n.name()
                    .to_lowercase()
                    .contains(&name_pattern.to_lowercase())
            })
            .unwrap_or(false)
    })
}

fn start_audio(input_device: cpal::Device) -> Result<AudioLoopback> {
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
    let buf_out = buf.clone();

    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    let clients_in = clients.clone();

    let err_fn = |e: cpal::StreamError| eprintln!("audio error: {e}");

    let input_stream = match in_cfg.sample_format() {
        SampleFormat::F32 => input_device.build_input_stream(
            &in_cfg.into(),
            move |data: &[f32], _| {
                let mono = downmix_f32(data, in_channels);
                push_to_buf(&mono, &buf_in);
                broadcast(&mono, &clients_in);
            },
            err_fn,
            None,
        )?,
        SampleFormat::I16 => input_device.build_input_stream(
            &in_cfg.into(),
            move |data: &[i16], _| {
                let mono = downmix_i16(data, in_channels);
                push_to_buf(&mono, &buf_in);
                broadcast(&mono, &clients_in);
            },
            err_fn,
            None,
        )?,
        SampleFormat::U16 => input_device.build_input_stream(
            &in_cfg.into(),
            move |data: &[u16], _| {
                let mono = downmix_u16(data, in_channels);
                push_to_buf(&mono, &buf_in);
                broadcast(&mono, &clients_in);
            },
            err_fn,
            None,
        )?,
        _ => anyhow::bail!("unsupported input sample format"),
    };

    let out_channels = out_cfg.channels() as usize;

    let output_stream = match out_cfg.sample_format() {
        SampleFormat::F32 => output_device.build_output_stream(
            &out_cfg.into(),
            move |data: &mut [f32], _| drain_to_output(data, out_channels, &buf_out),
            err_fn,
            None,
        )?,
        _ => anyhow::bail!("unsupported output sample format"),
    };

    input_stream.play()?;
    output_stream.play()?;

    Ok(AudioLoopback {
        _input: input_stream,
        _output: output_stream,
        clients,
        sample_rate,
    })
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
