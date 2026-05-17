use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::ui::components::spectrum::processor::{self, SpectrumBins};
use crate::ui::widgets::spectrum::SpectrumWidget;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct SpectrumComponent {
    _stream: Option<cpal::Stream>,
    bins: Option<SpectrumBins>,
    sample_rate: u32,
    is_stereo: Option<Arc<Mutex<bool>>>,
    pub device_name: Option<String>,
}

impl SpectrumComponent {
    pub fn inactive() -> Self {
        Self {
            _stream: None,
            bins: None,
            sample_rate: 0,
            is_stereo: None,
            device_name: None,
        }
    }

    pub fn is_active(&self) -> bool {
        self.bins.is_some()
    }

    pub fn start(
        &mut self,
        device_name: &str,
        iq_rate: u32,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<()> {
        let (stream, bins, is_stereo, name) =
            processor::start_iq_stream(device_name, iq_rate, errors)?;
        self._stream = Some(stream);
        self.bins = Some(bins);
        self.is_stereo = Some(is_stereo);
        self.sample_rate = iq_rate;
        self.device_name = Some(name);
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self._stream.take() {
            // cpal::Stream is !Send, but dropping it on a background thread is safe
            // when we hold exclusive ownership. Avoids blocking the TUI event loop on
            // ALSA/PipeWire teardown, which can take hundreds of milliseconds.
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

    pub fn render_to(&mut self, frame: &mut Frame, area: Rect, dc_suppress: bool) {
        let (Some(bins), Some(is_stereo)) = (&self.bins, &self.is_stereo) else {
            return;
        };
        frame.render_widget(
            SpectrumWidget::new(
                bins.clone(),
                self.sample_rate,
                is_stereo.clone(),
                dc_suppress,
            ),
            area,
        );
    }
}
