use std::collections::VecDeque;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};

type Clients = Arc<Mutex<Vec<SyncSender<Vec<f32>>>>>;

#[derive(Clone)]
pub enum AudioMode {
    Loopback,
}

// Dropping this signals the listener thread to exit within ~50 ms.
struct RemoteHandle {
    stop_tx: Sender<()>,
}

impl Drop for RemoteHandle {
    fn drop(&mut self) {
        let _ = self.stop_tx.send(());
    }
}

pub struct Audio {
    _mode: Option<AudioMode>,
    _audio_in: Option<cpal::Stream>,
    _audio_out: Option<cpal::Stream>,
    errors: Arc<Mutex<Vec<String>>>,
    device_name: Option<String>,
    rx_socket: PathBuf,
    // Shared with the audio input callback; remote listener adds senders here.
    clients: Clients,
    _remote_handle: Option<RemoteHandle>,
}

impl Audio {
    pub fn new(errors: Arc<Mutex<Vec<String>>>, rx_socket: PathBuf) -> Self {
        Self {
            _mode: None,
            _audio_in: None,
            _audio_out: None,
            errors,
            device_name: None,
            rx_socket,
            clients: Arc::new(Mutex::new(Vec::new())),
            _remote_handle: None,
        }
    }

    pub fn errors(&self) -> &Arc<Mutex<Vec<String>>> {
        &self.errors
    }

    pub fn is_active(&self) -> bool {
        self._audio_in.is_some()
    }

    pub fn is_remote_active(&self) -> bool {
        self._remote_handle.is_some()
    }

    pub fn active_device(&self) -> Option<&str> {
        self.device_name.as_deref()
    }

    pub fn rx_socket_path(&self) -> &str {
        self.rx_socket.to_str().unwrap_or("")
    }

    pub fn start_loopback(&mut self, device_name: &str) -> Result<()> {
        self.stop_audio();

        let device = Self::resolve_device(Some(device_name))
            .ok_or_else(|| anyhow::anyhow!("audio device not found: {device_name}"))?;

        let (audio_in, audio_out) = crate::app_utils::capture_stderr(
            || Self::build_loopback(device, self.clients.clone(), self.errors.clone()),
            &self.errors,
        )?;

        self._audio_in = Some(audio_in);
        self._audio_out = Some(audio_out);
        self._mode = Some(AudioMode::Loopback);
        self.device_name = Some(device_name.to_string());
        Ok(())
    }

    pub fn stop_audio(&mut self) {
        struct SendStream {
            _s: cpal::Stream,
        }
        unsafe impl Send for SendStream {}
        if let Some(s) = self._audio_in.take() {
            std::thread::spawn(move || drop(SendStream { _s: s }));
        }
        if let Some(s) = self._audio_out.take() {
            std::thread::spawn(move || drop(SendStream { _s: s }));
        }
        self._mode = None;
        self.device_name = None;
        // Audio gone → remote has nothing to forward.
        self.stop_remote();
    }

    pub fn start_remote(&mut self) -> Result<()> {
        if self._remote_handle.is_some() {
            return Ok(());
        }
        let _ = std::fs::remove_file(&self.rx_socket);
        let listener = UnixListener::bind(&self.rx_socket)?;
        listener.set_nonblocking(true)?;

        let (stop_tx, stop_rx) = mpsc::channel::<()>();
        let clients = self.clients.clone();

        std::thread::spawn(move || loop {
            match listener.accept() {
                Ok((mut socket, _)) => {
                    let (tx, rx) = mpsc::sync_channel::<Vec<f32>>(32);
                    if let Ok(mut c) = clients.lock() {
                        c.push(tx);
                    }
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
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if stop_rx.try_recv().is_ok() {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
                Err(_) => break,
            }
        });

        self._remote_handle = Some(RemoteHandle { stop_tx });
        Ok(())
    }

    pub fn stop_remote(&mut self) {
        self._remote_handle = None; // Drop sends stop signal; thread exits within ~50 ms.
        let _ = std::fs::remove_file(&self.rx_socket);
        if let Ok(mut c) = self.clients.lock() {
            c.clear();
        }
    }

    fn resolve_device(name_pattern: Option<&str>) -> Option<cpal::Device> {
        let host = cpal::default_host();
        match name_pattern {
            Some(pattern) => host.input_devices().ok()?.find(|d| {
                d.description()
                    .map(|n| n.name().to_lowercase() == pattern.to_lowercase())
                    .unwrap_or(false)
            }),
            None => host.default_input_device(),
        }
    }

    fn build_loopback(
        input_device: cpal::Device,
        clients: Clients,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<(cpal::Stream, cpal::Stream)> {
        let host = cpal::default_host();
        let output_device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("no output audio device found"))?;

        let in_cfg = input_device.default_input_config()?;
        let out_cfg = output_device.default_output_config()?;

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

        Ok((input_stream, output_stream))
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
