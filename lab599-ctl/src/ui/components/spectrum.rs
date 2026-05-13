use crate::app_state::AppState;
use crate::hardware::spectrum::SpectrumBins;
use crate::ui::components::component::Component;
use crate::ui::widgets::spectrum::SpectrumWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;
use std::sync::{Arc, Mutex};

pub struct SpectrumComponent {
    bins: Option<SpectrumBins>,
    sample_rate: u32,
    is_stereo: Option<Arc<Mutex<bool>>>,
}

impl SpectrumComponent {
    pub fn new() -> Self {
        Self {
            bins: None,
            sample_rate: 0,
            is_stereo: None,
        }
    }

    fn ensure_init(&mut self, app_state: &AppState) {
        if self.bins.is_some() {
            return;
        }
        let audio = &app_state.audio;
        if let (Some(bins), Some(is_stereo)) = (audio.bins(), audio.is_stereo()) {
            self.bins = Some(bins.clone());
            self.sample_rate = audio.iq_sample_rate();
            self.is_stereo = Some(is_stereo.clone());
        }
    }
}

impl Component for SpectrumComponent {
    fn constraint(&self) -> Constraint {
        if self.bins.is_some() {
            Constraint::Length(10)
        } else {
            Constraint::Length(0)
        }
    }

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
            SpectrumWidget {
                bins: bins.clone(),
                sample_rate: self.sample_rate,
                is_stereo: is_stereo.clone(),
                dc_suppress: app_state.radio.state().dc_suppress,
            },
            area,
        );
    }
}
