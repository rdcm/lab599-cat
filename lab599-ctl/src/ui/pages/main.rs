use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::page::Page;
use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::components::radio_info::RadioInfoComponent;
use crate::ui::components::smeter::SmeterComponent;
use crate::ui::components::status_flags::StatusFlagsComponent;
use crate::ui::ui_utils::apply_key;
use crate::ui::widgets::spectrum::SpectrumWidget;

pub struct MainPage {
    info: RadioInfoComponent,
    smeter: SmeterComponent,
    flags: StatusFlagsComponent,
}

impl MainPage {
    pub fn new() -> Self {
        Self {
            info: RadioInfoComponent,
            smeter: SmeterComponent,
            flags: StatusFlagsComponent,
        }
    }
}

impl Page for MainPage {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &mut AppState,
        key: Option<KeyEvent>,
    ) {
        let spectrum_height = if app_state.spectrum.is_active() {
            10
        } else {
            0
        };
        let areas = Layout::vertical([
            Constraint::Length(9),
            Constraint::Length(4),
            Constraint::Length(4),
            Constraint::Length(spectrum_height),
        ])
        .split(area);

        self.info.render(frame, areas[0], app_state, None);
        self.smeter.render(frame, areas[1], app_state, None);
        self.flags.render(frame, areas[2], app_state, None);

        let dc_suppress = app_state.radio.state().dc_suppress;
        if let (Some(bins), Some(is_stereo)) =
            (app_state.spectrum.bins(), app_state.spectrum.is_stereo())
        {
            frame.render_widget(
                SpectrumWidget::new(
                    bins,
                    app_state.spectrum.sample_rate(),
                    is_stereo,
                    dc_suppress,
                ),
                areas[3],
            );
        }

        if let Some(k) = key {
            if k.code != KeyCode::Char('i') {
                apply_key(k, &mut app_state.radio);
            }
        }
    }
}
