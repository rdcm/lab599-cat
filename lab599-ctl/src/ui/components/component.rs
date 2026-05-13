use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Rect},
    Frame,
};

use crate::app_state::AppState;

pub trait Component {
    fn constraint(&self) -> Constraint;
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        key: Option<KeyEvent>,
    );
}
