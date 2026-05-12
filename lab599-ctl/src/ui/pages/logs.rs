use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use super::page::{Action, Page};
use crate::hardware::state::RadioState;
use crate::ui::components::component::Component;
use crate::ui::components::error_log::ErrorLogComponent;

pub struct LogsPage {
    log: ErrorLogComponent,
}

impl LogsPage {
    pub fn new() -> Self {
        Self {
            log: ErrorLogComponent,
        }
    }
}

impl Page for LogsPage {
    fn name(&self) -> &'static str {
        "Logs"
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &RadioState,
        _key: Option<KeyEvent>,
    ) -> Option<Action> {
        self.log.render(frame, area, state, None);
        None
    }
}
