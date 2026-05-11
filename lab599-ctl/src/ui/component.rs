use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Rect},
    Frame,
};

use crate::{events::bus::EventBus, state::RadioState};

pub trait Component {
    fn constraint(&self) -> Constraint;
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState);
    fn handle_event(&mut self, _event: &KeyEvent, _bus: &mut EventBus) {}
}
