use crate::hardware::state::RadioState;
use crate::ui::components::component::Component;
use crate::ui::widgets::radio_info::RadioInfoWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;

pub struct RadioInfoComponent;

impl Component for RadioInfoComponent {
    fn constraint(&self) -> Constraint {
        Constraint::Length(9)
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &RadioState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(RadioInfoWidget::from(state), area);
    }
}
