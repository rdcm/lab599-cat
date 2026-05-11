use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Layout, Rect},
    Frame,
};

use crate::{
    events::{action::Action, bus::EventBus},
    spectrum::IqCapture,
    state::RadioState,
    ui::{
        component::Component,
        components::{
            radio_info::RadioInfo, smeter::Smeter, spectrum::Spectrum, status_flags::StatusFlags,
        },
    },
};

use super::page::Page;

pub struct MainPage {
    components: Vec<Box<dyn Component>>,
}

impl MainPage {
    pub fn new(iq: Option<&IqCapture>) -> Self {
        let components: Vec<Box<dyn Component>> = vec![
            Box::new(RadioInfo),
            Box::new(Smeter),
            Box::new(StatusFlags),
            Box::new(Spectrum::new(iq)),
        ];
        Self { components }
    }
}

impl Page for MainPage {
    fn name(&self) -> &str {
        "Main"
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let constraints: Vec<_> = self.components.iter().map(|c| c.constraint()).collect();
        let areas = Layout::vertical(constraints).split(area);
        for (component, area) in self.components.iter_mut().zip(areas.iter()) {
            component.render(frame, *area, state);
        }
    }

    fn handle_event(&mut self, event: &KeyEvent, bus: &mut EventBus) {
        for component in &mut self.components {
            component.handle_event(event, bus);
        }

        let action = match (event.code, event.modifiers) {
            (KeyCode::Right, _) => Action::TuneStep(1),
            (KeyCode::Left, _) => Action::TuneStep(-1),
            (KeyCode::Up, _) => Action::StepNext,
            (KeyCode::Down, _) => Action::StepPrev,
            (KeyCode::PageUp, _) | (KeyCode::Char('+'), _) => Action::Tune(1_000_000),
            (KeyCode::PageDown, _) | (KeyCode::Char('-'), _) => Action::Tune(-1_000_000),
            (KeyCode::Char('m'), _) => Action::ToggleMode,
            (KeyCode::Char('f'), _) => Action::ToggleFilter,
            (KeyCode::Char('t'), _) => Action::TogglePtt,
            (KeyCode::Char('p'), _) => Action::TogglePreamp,
            (KeyCode::Char('a'), _) => Action::ToggleAttenuator,
            (KeyCode::Char('s'), _) => Action::ToggleSplit,
            (KeyCode::Char('c'), _) => Action::ToggleCmr,
            (KeyCode::Char('v'), _) => Action::ToggleVox,
            (KeyCode::Char('n'), _) => Action::ToggleNr,
            (KeyCode::Char('b'), _) => Action::ToggleNb,
            (KeyCode::Char('x'), _) => Action::ToggleNotch,
            (KeyCode::Char('o'), _) => Action::ToggleMon,
            (KeyCode::Char('d'), _) => Action::ToggleDif,
            (KeyCode::Char('z'), _) => Action::ToggleDcSuppress,
            (KeyCode::Char('['), _) => Action::BandDown,
            (KeyCode::Char(']'), _) => Action::BandUp,
            _ => return,
        };
        bus.publish(action);
    }
}
