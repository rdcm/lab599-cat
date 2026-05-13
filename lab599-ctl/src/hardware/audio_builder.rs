use crate::hardware::audio::{Audio, AudioMode};
use crate::hardware::spectrum;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct AudioBuilder {
    transport_mode: Option<AudioMode>,
    audio_device: Option<String>,
    rx_socket: Option<PathBuf>,
    iq_device: Option<String>,
    iq_rate: u32,
}

impl AudioBuilder {
    pub fn new() -> Self {
        Self {
            transport_mode: None,
            audio_device: None,
            rx_socket: None,
            iq_device: None,
            iq_rate: 192_000,
        }
    }

    pub fn with_loopback(mut self) -> Self {
        self.transport_mode = Some(AudioMode::Loopback);
        self
    }

    pub fn with_remote(mut self, device: &str, rx_socket: &Path) -> Self {
        self.transport_mode = Some(AudioMode::Remote);
        self.audio_device = Some(device.to_owned());
        self.rx_socket = Some(rx_socket.to_owned());
        self
    }

    pub fn with_iq(mut self, device: Option<&str>, rate: u32) -> Self {
        self.iq_device = device.map(str::to_owned);
        self.iq_rate = rate;
        self
    }

    pub fn build(self, mut on_err: impl FnMut(String)) -> Audio {
        let errors: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        let mut audio_in = None;
        let mut audio_out = None;
        let mut mode = None;

        if let Some(transport_mode) = &self.transport_mode {
            match Audio::build_transport(
                self.audio_device.as_deref(),
                self.rx_socket.as_deref(),
                transport_mode,
                errors.clone(),
            ) {
                Ok((input, output)) => {
                    audio_in = Some(input);
                    audio_out = output;
                    mode = self.transport_mode.clone();
                }
                Err(e) => eprintln!("audio error: {e}"),
            }
        }

        let mut iq_in = None;
        let mut bins = None;
        let mut is_stereo = None;
        let mut iq_sample_rate = 0;

        if let Some(iq_name) = self.iq_device.as_deref() {
            match spectrum::start_iq_stream(iq_name, self.iq_rate, errors.clone()) {
                Ok((stream, b, stereo)) => {
                    iq_in = Some(stream);
                    bins = Some(b);
                    is_stereo = Some(stereo);
                    iq_sample_rate = self.iq_rate;
                }
                Err(e) => on_err(e.to_string()),
            }
        }

        Audio::new(
            mode,
            audio_in,
            audio_out,
            iq_in,
            iq_sample_rate,
            bins,
            is_stereo,
            errors,
        )
    }
}
