use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::app_state::AppState;

pub trait Page {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &mut AppState,
        key: Option<KeyEvent>,
    );
}
