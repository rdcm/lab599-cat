use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use crate::app_state::AppState;
use crate::ui::layout::AppLayout;
use crate::ui::pages::{help::HelpPage, logs::LogsPage, main::MainPage, page::Page};

pub struct Router {
    pages: Vec<Box<dyn Page>>,
    current: usize,
}

impl Router {
    pub fn new() -> Self {
        let pages: Vec<Box<dyn Page>> = vec![
            Box::new(MainPage::new()),
            Box::new(HelpPage::new()),
            Box::new(LogsPage::new()),
        ];
        Self { pages, current: 0 }
    }

    pub fn render(&mut self, f: &mut Frame, state: &mut AppState, key: Option<KeyEvent>) {
        if matches!(key, Some(k) if k.code == KeyCode::Tab) {
            self.current = (self.current + 1) % self.pages.len();
            return;
        }
        let names: Vec<&'static str> = self.pages.iter().map(|p| p.name()).collect();
        let outlet = AppLayout::render(f, state.radio.state(), &names, self.current);
        if let Some(page) = self.pages.get_mut(self.current) {
            page.render(f, outlet, state, key);
        }
    }
}
