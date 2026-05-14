use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::widgets::app_bar::AppBarWidget;

pub struct AppBarComponent {
    tabs: Vec<&'static str>,
    current: usize,
}

impl AppBarComponent {
    pub fn new(tabs: Vec<&'static str>) -> Self {
        Self { tabs, current: 0 }
    }

    pub fn set_current(&mut self, current: usize) {
        self.current = current;
    }
}

impl Component for AppBarComponent {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        _key: Option<KeyEvent>,
    ) {
        let model = app_state.radio.state().model.to_string();
        frame.render_widget(AppBarWidget::new(model, &self.tabs, self.current), area);
    }
}
