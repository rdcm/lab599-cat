use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::widgets::smeter::SmeterWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct SmeterComponent;

impl Component for SmeterComponent {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(SmeterWidget::from(app_state.radio.state()), area);
    }
}
