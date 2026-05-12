use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Rect},
    Frame,
};

use crate::hardware::state::RadioState;

pub trait Component {
    fn constraint(&self) -> Constraint;
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState, key: Option<KeyEvent>);
}
