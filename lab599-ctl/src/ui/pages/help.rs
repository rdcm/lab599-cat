use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::page::Page;
use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::components::radio_help::RadioHelpComponent;
use crate::ui::components::tui_help::TuiHelpComponent;

pub struct HelpPage {
    radio_help: RadioHelpComponent,
    tui_help: TuiHelpComponent,
}

impl HelpPage {
    pub fn new() -> Self {
        Self {
            radio_help: RadioHelpComponent,
            tui_help: TuiHelpComponent,
        }
    }
}

impl Page for HelpPage {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &mut AppState,
        _key: Option<KeyEvent>,
    ) {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(area);

        self.radio_help.render(frame, left, app_state, None);
        self.tui_help.render(frame, right, app_state, None);
    }
}
