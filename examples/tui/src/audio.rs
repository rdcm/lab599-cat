use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};

pub struct AudioLoopback {
    _input: cpal::Stream,
    _output: cpal::Stream,
}

pub fn list_audio_devices() {
    let host = cpal::default_host();
    println!("Available audio input devices:");
    if let Ok(devices) = host.input_devices() {
        for d in devices {
            if let Ok(name) = d.name() {
                println!("  {name}");
            }
        }
    }
}

pub fn find_audio_device(name_pattern: &str) -> Option<cpal::Device> {
    let host = cpal::default_host();
    host.input_devices().ok()?.find(|d| {
        d.name()
            .map(|n| n.to_lowercase().contains(&name_pattern.to_lowercase()))
            .unwrap_or(false)
    })
}

pub fn start_audio(input_device: cpal::Device) -> Result<AudioLoopback> {
    let host = cpal::default_host();
    let output_device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("no output audio device found"))?;

    let in_cfg = input_device.default_input_config()?;
    let out_cfg = output_device.default_output_config()?;

    let buf: Arc<Mutex<VecDeque<f32>>> = Arc::new(Mutex::new(VecDeque::new()));
    let buf_in = buf.clone();
    let buf_out = buf.clone();

    let in_channels = in_cfg.channels() as usize;
    let err_fn = |e: cpal::StreamError| eprintln!("audio error: {e}");

    let input_stream = match in_cfg.sample_format() {
        SampleFormat::F32 => input_device.build_input_stream(
            &in_cfg.into(),
            move |data: &[f32], _| push_mono(data, in_channels, &buf_in),
            err_fn,
            None,
        )?,
        SampleFormat::I16 => input_device.build_input_stream(
            &in_cfg.into(),
            move |data: &[i16], _| {
                let f: Vec<f32> =
                    data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                push_mono(&f, in_channels, &buf_in);
            },
            err_fn,
            None,
        )?,
        SampleFormat::U16 => input_device.build_input_stream(
            &in_cfg.into(),
            move |data: &[u16], _| {
                let f: Vec<f32> = data
                    .iter()
                    .map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0)
                    .collect();
                push_mono(&f, in_channels, &buf_in);
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

    Ok(AudioLoopback { _input: input_stream, _output: output_stream })
}

fn push_mono(data: &[f32], channels: usize, buf: &Mutex<VecDeque<f32>>) {
    let mono = data
        .chunks(channels)
        .map(|ch| ch.iter().sum::<f32>() / ch.len() as f32);
    if let Ok(mut b) = buf.lock() {
        b.extend(mono);
        let len = b.len();
        if len > 96_000 {
            b.drain(..len - 96_000);
        }
    }
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
