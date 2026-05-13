use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::app_state::AppState;

pub trait Page {
    fn name(&self) -> &'static str;
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &AppState,
        key: Option<KeyEvent>,
    ) -> Option<Action>;
}

pub enum Action {
    TuneStep(i8),
    Tune(i64),
    StepNext,
    StepPrev,
    ToggleMode,
    ToggleFilter,
    TogglePtt,
    TogglePreamp,
    ToggleAttenuator,
    ToggleSplit,
    ToggleCmr,
    ToggleVox,
    ToggleNr,
    ToggleNb,
    ToggleNotch,
    ToggleMon,
    ToggleDif,
    ToggleDcSuppress,
    BandUp,
    BandDown,
}
