use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::widgets::radio_help::RadioHelpWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct RadioHelpComponent;

impl Component for RadioHelpComponent {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(RadioHelpWidget::from(app_state.radio.state()), area);
    }
}
