use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::widgets::error_log::ErrorLogWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Rect};
use ratatui::Frame;

pub struct ErrorLogComponent;

impl Component for ErrorLogComponent {
    fn constraint(&self) -> Constraint {
        Constraint::Fill(1)
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(ErrorLogWidget::from(app_state.radio.state()), area);
    }
}
