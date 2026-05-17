use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{Block, Borders};
use ratatui::Frame;

use crate::app_state::AppState;
use crate::ui::components::app_bar::AppBarComponent;
use crate::ui::components::component::Component;
use crate::ui::components::spectrum::component::SpectrumComponent;
use crate::ui::pages::page::Page;
use crate::ui::pages::{help::HelpPage, logs::LogsPage, main::MainPage};

const LOGS_PAGE: usize = 2;

pub struct AppLayout {
    app_bar: AppBarComponent,
    pages: Vec<(&'static str, Box<dyn Page>)>,
    current: usize,
    last_log_len: usize,
    page_needs_draw: bool,
}

impl AppLayout {
    pub fn new(spectrum: SpectrumComponent) -> Self {
        let pages: Vec<(&'static str, Box<dyn Page>)> = vec![
            ("Main", Box::new(MainPage::new(spectrum))),
            ("Help", Box::new(HelpPage::new())),
            ("Logs", Box::new(LogsPage::new())),
        ];
        let tabs = pages.iter().map(|(name, _)| *name).collect();
        Self {
            app_bar: AppBarComponent::new(tabs),
            pages,
            current: 0,
            last_log_len: 0,
            page_needs_draw: true,
        }
    }

    /// Returns true when a redraw is needed.
    /// For the Logs page this is only when the error list has grown.
    /// All other pages always need a redraw (live data).
    pub fn needs_draw(&self, state: &AppState) -> bool {
        if self.page_needs_draw {
            return true;
        }
        if self.current == LOGS_PAGE {
            state.radio.state().errors.len() != self.last_log_len
        } else {
            true
        }
    }

    pub fn render(&mut self, f: &mut Frame, state: &mut AppState, key: Option<KeyEvent>) {
        if matches!(key, Some(k) if k.code == KeyCode::Tab) {
            self.current = (self.current + 1) % self.pages.len();
            self.page_needs_draw = true;
            return;
        }

        self.page_needs_draw = false;
        if self.current == LOGS_PAGE {
            self.last_log_len = state.radio.state().errors.len();
        }

        self.app_bar.set_current(self.current);
        let outlet = Block::default().borders(Borders::ALL).inner(f.area());
        self.app_bar.render(f, f.area(), state, None);
        if let Some((_, page)) = self.pages.get_mut(self.current) {
            page.render(f, outlet, state, key);
        }
    }
}
