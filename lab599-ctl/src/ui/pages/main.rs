use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Layout, Rect},
    Frame,
};

use super::page::{Action, Page};
use crate::app_state::AppState;
use crate::ui::components::component::Component;
use crate::ui::components::radio_info::RadioInfoComponent;
use crate::ui::components::smeter::SmeterComponent;
use crate::ui::components::spectrum::SpectrumComponent;
use crate::ui::components::status_flags::StatusFlagsComponent;
use crate::ui::utils::map_key;

pub struct MainPage {
    info: RadioInfoComponent,
    smeter: SmeterComponent,
    flags: StatusFlagsComponent,
    spectrum: SpectrumComponent,
}

impl MainPage {
    pub fn new() -> Self {
        Self {
            info: RadioInfoComponent,
            smeter: SmeterComponent,
            flags: StatusFlagsComponent,
            spectrum: SpectrumComponent::new(),
        }
    }
}

impl Page for MainPage {
    fn name(&self) -> &'static str {
        "Main"
    }

    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        key: Option<KeyEvent>,
    ) -> Option<Action> {
        let areas = Layout::vertical([
            self.info.constraint(),
            self.smeter.constraint(),
            self.flags.constraint(),
            self.spectrum.constraint(),
        ])
        .split(area);

        self.info.render(frame, areas[0], app_state, None);
        self.smeter.render(frame, areas[1], app_state, None);
        self.flags.render(frame, areas[2], app_state, None);
        self.spectrum.render(frame, areas[3], app_state, None);

        key.and_then(map_key)
    }
}
