use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::widgets::status_flags::StatusFlagsWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct StatusFlagsComponent;

impl Component for StatusFlagsComponent {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(StatusFlagsWidget::from(app_state.radio.state()), area);
    }
}
