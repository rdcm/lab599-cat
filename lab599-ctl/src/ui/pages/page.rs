use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::hardware::state::RadioState;

pub trait Page {
    fn name(&self) -> &'static str;
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        state: &RadioState,
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
