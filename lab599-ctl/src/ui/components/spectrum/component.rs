use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::components::spectrum::processor::{self, SpectrumBins};
use crate::ui::widgets::spectrum::SpectrumWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct SpectrumComponent {
    _stream: Option<cpal::Stream>,
    bins: Option<SpectrumBins>,
    sample_rate: u32,
    is_stereo: Option<Arc<Mutex<bool>>>,
}

impl SpectrumComponent {
    pub fn inactive() -> Self {
        Self {
            _stream: None,
            bins: None,
            sample_rate: 0,
            is_stereo: None,
        }
    }

    pub fn start(
        device: &str,
        sample_rate: u32,
        errors: Arc<Mutex<Vec<String>>>,
    ) -> Result<Self> {
        let (stream, bins, is_stereo) =
            processor::start_iq_stream(device, sample_rate, errors)?;
        Ok(Self {
            _stream: Some(stream),
            bins: Some(bins),
            sample_rate,
            is_stereo: Some(is_stereo),
        })
    }

    pub fn is_active(&self) -> bool {
        self.bins.is_some()
    }
}

impl Component for SpectrumComponent {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        let (Some(bins), Some(is_stereo)) = (&self.bins, &self.is_stereo) else {
            return;
        };
        frame.render_widget(
            SpectrumWidget::new(
                bins.clone(),
                self.sample_rate,
                is_stereo.clone(),
                app_state.radio.state().dc_suppress,
            ),
            area,
        );
    }
}
