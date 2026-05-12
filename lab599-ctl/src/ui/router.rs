use crate::hardware::spectrum::IqCapture;
use crate::ui::pages::{help::HelpPage, logs::LogsPage, main::MainPage, page::Page};

pub struct Router {
    pages: Vec<Box<dyn Page>>,
    current: usize,
}

impl Router {
    pub fn new(iq: Option<&IqCapture>) -> Self {
        let pages: Vec<Box<dyn Page>> = vec![
            Box::new(MainPage::new(iq)),
            Box::new(HelpPage::new()),
            Box::new(LogsPage::new()),
        ];
        Self { pages, current: 0 }
    }

    pub fn page_names(&self) -> Vec<&'static str> {
        self.pages.iter().map(|p| p.name()).collect()
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn advance_page(&mut self) {
        self.current = (self.current + 1) % self.pages.len();
    }

    pub fn current_page_mut(&mut self) -> Option<&mut (dyn Page + 'static)> {
        self.pages.get_mut(self.current).map(|p| p.as_mut())
    }
}
