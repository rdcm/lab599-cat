use crate::hardware::state::RadioState;
use crate::ui::components::component::Component;
use crate::ui::widgets::status_flags::StatusFlagsWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;

pub struct StatusFlagsComponent;

impl Component for StatusFlagsComponent {
    fn constraint(&self) -> Constraint {
        Constraint::Length(4)
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &RadioState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(StatusFlagsWidget::from(state), area);
    }
}
