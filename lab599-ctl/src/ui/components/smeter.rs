use crate::app_state::AppState;
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
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(SmeterView::from(app_state.radio.state()), area);
    }
}
