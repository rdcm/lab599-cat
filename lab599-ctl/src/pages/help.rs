use ratatui::{
    layout::{Layout, Rect},
    Frame,
};

use crate::{
    state::RadioState,
    ui::{
        component::Component,
        components::{radio_help::RadioHelp, tui_help::TuiHelp},
    },
};

use super::page::Page;

pub struct HelpPage {
    radio: RadioHelp,
    tui: TuiHelp,
}

impl HelpPage {
    pub fn new() -> Self {
        Self {
            radio: RadioHelp,
            tui: TuiHelp,
        }
    }
}

impl Page for HelpPage {
    fn name(&self) -> &str {
        "Help"
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let areas =
            Layout::horizontal([self.radio.constraint(), self.tui.constraint()]).split(area);

        self.radio.render(frame, areas[0], state);
        self.tui.render(frame, areas[1], state);
    }
}
