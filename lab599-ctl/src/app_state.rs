use std::time::Duration;

use crate::hardware::audio::Audio;
use crate::hardware::radio::Radio;
use crate::ui::components::spectrum::component::SpectrumComponent;

pub struct AppState {
    pub radio: Radio,
    pub audio: Audio,
    pub spectrum: SpectrumComponent,
    pub iq_rate: u32,
    pub poll_interval: Duration,
}
