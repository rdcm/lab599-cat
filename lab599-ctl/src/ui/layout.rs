use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Layout};
use ratatui::Frame;

use crate::app_state::AppState;
use crate::ui::components::app_bar::AppBarComponent;
use crate::ui::components::component::Component;
use crate::ui::pages::page::Page;
use crate::ui::pages::{help::HelpPage, logs::LogsPage, main::MainPage};

pub struct AppLayout {
    app_bar: AppBarComponent,
    pages: Vec<(&'static str, Box<dyn Page>)>,
    current: usize,
}

impl AppLayout {
    pub fn new() -> Self {
        let pages: Vec<(&'static str, Box<dyn Page>)> = vec![
            ("Main", Box::new(MainPage::new())),
            ("Help", Box::new(HelpPage::new())),
            ("Logs", Box::new(LogsPage::new())),
        ];
        let tabs = pages.iter().map(|(name, _)| *name).collect();
        Self {
            app_bar: AppBarComponent::new(tabs),
            pages,
            current: 0,
        }
    }

    pub fn render(&mut self, f: &mut Frame, state: &mut AppState, key: Option<KeyEvent>) {
        if matches!(key, Some(k) if k.code == KeyCode::Tab) {
            self.current = (self.current + 1) % self.pages.len();
            return;
        }
        self.app_bar.set_current(self.current);
        let [bar_area, outlet] =
            Layout::vertical([Constraint::Length(2), Constraint::Fill(1)]).areas(f.area());
        self.app_bar.render(f, bar_area, state, None);
        if let Some((_, page)) = self.pages.get_mut(self.current) {
            page.render(f, outlet, state, key);
        }
    }
}
