use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;

use super::{help::HelpPage, logs::LogsPage, main::MainPage, page::Page};
use crate::pages::layout::AppLayout;
use crate::{
    events::{action::Action, bus::EventBus},
    spectrum::IqCapture,
    state::RadioState,
};

pub struct Router {
    pages: Vec<Box<dyn Page>>,
    current: usize,
}

impl Router {
    pub fn new(iq: Option<&IqCapture>) -> Self {
        let pages: Vec<Box<dyn Page>> = vec![
            Box::new(MainPage::new(iq)),
            Box::new(HelpPage::new()),
            Box::new(LogsPage),
        ];
        Self { pages, current: 0 }
    }

    pub fn render(&mut self, frame: &mut Frame, state: &RadioState) {
        let labels: Vec<&str> = self.pages.iter().map(|p| p.name()).collect();
        let inner = AppLayout::render(frame, state, &labels, self.current);

        if let Some(page) = self.pages.get_mut(self.current) {
            page.render(frame, inner, state);
        }
    }

    pub fn handle_event(&mut self, event: &KeyEvent, bus: &mut EventBus) {
        if matches!(
            (event.code, event.modifiers),
            (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL)
        ) {
            bus.publish(Action::Quit);
            return;
        }
        if event.code == KeyCode::Tab {
            self.current = (self.current + 1) % self.pages.len();
            return;
        }
        if let Some(page) = self.pages.get_mut(self.current) {
            page.handle_event(event, bus);
        }
    }
}
