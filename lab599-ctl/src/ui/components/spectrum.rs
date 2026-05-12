use crate::hardware::spectrum::{IqCapture, SpectrumBins};
use crate::hardware::state::RadioState;
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
    pub fn new(iq: Option<&IqCapture>) -> Self {
        match iq {
            Some(capture) => Self {
                bins: Some(capture.bins.clone()),
                sample_rate: capture.sample_rate,
                is_stereo: Some(capture.is_stereo.clone()),
            },
            None => Self {
                bins: None,
                sample_rate: 0,
                is_stereo: None,
            },
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
        state: &RadioState,
        _key: Option<KeyEvent>,
    ) {
        let (Some(bins), Some(is_stereo)) = (&self.bins, &self.is_stereo) else {
            return;
        };
        frame.render_widget(
            SpectrumWidget {
                bins: bins.clone(),
                sample_rate: self.sample_rate,
                is_stereo: is_stereo.clone(),
                dc_suppress: state.dc_suppress,
            },
            area,
        );
    }
}
