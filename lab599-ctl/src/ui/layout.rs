use ratatui::{layout::Rect, Frame};

use crate::hardware::radio_state::RadioState;
use crate::ui::widgets::app_bar::AppBar;

pub struct AppLayout;

impl AppLayout {
    pub fn render(
        frame: &mut Frame,
        state: &RadioState,
        tabs: &[&'static str],
        current: usize,
    ) -> Rect {
        AppBar::render(frame, state, tabs, current)
    }
}
