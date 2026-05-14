use crate::hardware::audio::{Audio, AudioMode};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct AudioBuilder {
    transport_mode: Option<AudioMode>,
    audio_device: Option<String>,
    rx_socket: Option<PathBuf>,
}

impl AudioBuilder {
    pub fn new() -> Self {
        Self {
            transport_mode: None,
            audio_device: None,
            rx_socket: None,
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

    pub fn build(self, _on_err: impl FnMut(String)) -> Audio {
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

        Audio::new(mode, audio_in, audio_out, errors)
    }
}
