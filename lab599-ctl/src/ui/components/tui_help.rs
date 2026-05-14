use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::widgets::tui_help::TuiHelpWidget;
use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

pub struct TuiHelpComponent;

impl Component for TuiHelpComponent {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        frame.render_widget(TuiHelpWidget::from(app_state.radio.state()), area);
    }
}
