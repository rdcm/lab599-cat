use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Layout, Rect},
    Frame,
};

use super::page::{Action, Page};
use crate::hardware::spectrum::IqCapture;
use crate::hardware::state::RadioState;
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
    pub fn new(iq: Option<&IqCapture>) -> Self {
        Self {
            info: RadioInfoComponent,
            smeter: SmeterComponent,
            flags: StatusFlagsComponent,
            spectrum: SpectrumComponent::new(iq),
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
        state: &RadioState,
        key: Option<KeyEvent>,
    ) -> Option<Action> {
        let areas = Layout::vertical([
            self.info.constraint(),
            self.smeter.constraint(),
            self.flags.constraint(),
            self.spectrum.constraint(),
        ])
        .split(area);

        self.info.render(frame, areas[0], state, None);
        self.smeter.render(frame, areas[1], state, None);
        self.flags.render(frame, areas[2], state, None);
        self.spectrum.render(frame, areas[3], state, None);

        key.and_then(map_key)
    }
}
