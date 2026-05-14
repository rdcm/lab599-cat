use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::components::spectrum::processor::{self, SpectrumBins};
use crate::ui::widgets::spectrum::SpectrumWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::sync::{Arc, Mutex};

pub struct SpectrumComponent {
    _stream: Option<cpal::Stream>,
    bins: Option<SpectrumBins>,
    sample_rate: u32,
    is_stereo: Option<Arc<Mutex<bool>>>,
}

impl SpectrumComponent {
    pub fn new() -> Self {
        Self {
            _stream: None,
            bins: None,
            sample_rate: 0,
            is_stereo: None,
        }
    }

    fn ensure_init(&mut self, app_state: &AppState) {
        if self.bins.is_some() {
            return;
        }
        let Some(device) = app_state._config.iq_device.as_deref() else {
            return;
        };
        match processor::start_iq_stream(
            device,
            app_state._config.iq_rate,
            app_state.audio.errors().clone(),
        ) {
            Ok((stream, bins, is_stereo)) => {
                self._stream = Some(stream);
                self.bins = Some(bins);
                self.sample_rate = app_state._config.iq_rate;
                self.is_stereo = Some(is_stereo);
            }
            Err(e) => {
                if let Ok(mut errs) = app_state.audio.errors().lock() {
                    errs.push(e.to_string());
                }
            }
        }
    }
}

impl SpectrumComponent {
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
        self.ensure_init(app_state);
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
