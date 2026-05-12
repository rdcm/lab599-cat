use crate::hardware::state::RadioState;
use crate::ui::components::component::Component;
use crate::ui::widgets::smeter::SmeterView;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;

pub struct SmeterComponent;

impl Component for SmeterComponent {
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
        frame.render_widget(SmeterView::from(state), area);
    }
}
