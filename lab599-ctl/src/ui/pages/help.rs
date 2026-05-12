use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use super::page::{Action, Page};
use crate::hardware::state::RadioState;
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
    fn name(&self) -> &'static str {
        "Help"
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &RadioState,
        _key: Option<KeyEvent>,
    ) -> Option<Action> {
        let [left, right] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(area);

        self.radio_help.render(frame, left, state, None);
        self.tui_help.render(frame, right, state, None);
        None
    }
}
