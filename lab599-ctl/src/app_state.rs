use crate::config::Config;
use crate::hardware::audio::Audio;
use crate::hardware::radio::Radio;

pub struct AppState {
    pub radio: Radio,
    pub audio: Audio,
    pub _config: Config,
}
