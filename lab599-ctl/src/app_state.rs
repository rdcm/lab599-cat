use crate::app_config::AppConfig;
use crate::hardware::audio::Audio;
use crate::hardware::radio::Radio;
use crate::services::spectrum::Spectrum;

pub struct AppState {
    pub radio: Radio,
    pub audio: Audio,
    pub spectrum: Spectrum,
    pub config: AppConfig,
}
