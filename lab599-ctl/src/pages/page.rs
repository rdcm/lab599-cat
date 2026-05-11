use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::{events::bus::EventBus, state::RadioState};

pub trait Page {
    fn name(&self) -> &str;
    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState);
    fn handle_event(&mut self, _event: &KeyEvent, _bus: &mut EventBus) {}
}
