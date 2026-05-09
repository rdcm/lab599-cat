use std::time::Instant;

use lab599_cat::Mode;

pub const MAX_ERRORS: usize = 8;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Hz10,
    Hz100,
    Hz500,
    #[default]
    K1,
    K2_5,
    K5,
    K10,
}

impl Step {
    pub fn hz(self) -> u64 {
        match self {
            Step::Hz10 => 10,
            Step::Hz100 => 100,
            Step::Hz500 => 500,
            Step::K1 => 1_000,
            Step::K2_5 => 2_500,
            Step::K5 => 5_000,
            Step::K10 => 10_000,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Step::Hz10 => Step::Hz100,
            Step::Hz100 => Step::Hz500,
            Step::Hz500 => Step::K1,
            Step::K1 => Step::K2_5,
            Step::K2_5 => Step::K5,
            Step::K5 => Step::K10,
            Step::K10 => Step::Hz10,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Step::Hz10 => Step::K10,
            Step::Hz100 => Step::Hz10,
            Step::Hz500 => Step::Hz100,
            Step::K1 => Step::Hz500,
            Step::K2_5 => Step::K1,
            Step::K5 => Step::K2_5,
            Step::K10 => Step::K5,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Step::Hz10 => "10 Hz",
            Step::Hz100 => "100 Hz",
            Step::Hz500 => "500 Hz",
            Step::K1 => "1 kHz",
            Step::K2_5 => "2.5 kHz",
            Step::K5 => "5 kHz",
            Step::K10 => "10 kHz",
        }
    }
}

#[derive(Clone)]
pub struct RadioState {
    pub model: String,
    pub frequency: u64,
    pub mode: Option<Mode>,
    pub filter: u8,
    pub smeter: u16,
    pub ptt: bool,
    pub preamp: bool,
    pub attenuator: bool,
    pub split: bool,
    pub cmr: bool,
    pub step: Step,
    pub audio_active: bool,
    pub dc_suppress: bool,
    pub errors: Vec<(Instant, String)>,
    // extended state
    pub vox: bool,
    pub nr: bool,
    pub nb: bool,
    pub notch: bool,
    pub mon: bool,
    pub dif: bool,
    pub power: u8,
    pub af_gain: u16,
    pub voltage: u16,
    pub swr: u16,
    pub busy: bool,
}

impl Default for RadioState {
    fn default() -> Self {
        Self {
            model: String::new(),
            frequency: 0,
            mode: None,
            filter: 0,
            smeter: 0,
            ptt: false,
            preamp: false,
            attenuator: false,
            split: false,
            cmr: false,
            step: Step::default(),
            audio_active: false,
            dc_suppress: true,
            errors: Vec::new(),
            vox: false,
            nr: false,
            nb: false,
            notch: false,
            mon: false,
            dif: false,
            power: 0,
            af_gain: 0,
            voltage: 0,
            swr: 0,
            busy: false,
        }
    }
}

impl RadioState {
    pub fn log_error(&mut self, msg: String) {
        self.errors.insert(0, (Instant::now(), msg));
        self.errors.truncate(MAX_ERRORS);
    }

    pub fn mode_str(&self) -> &'static str {
        match self.mode {
            Some(Mode::Lsb) => "LSB",
            Some(Mode::Usb) => "USB",
            Some(Mode::Cw) => "CW",
            Some(Mode::CwR) => "CW-R",
            Some(Mode::Fm) => "FM",
            Some(Mode::Am) => "AM",
            Some(Mode::Dig) => "DIG",
            None => "---",
        }
    }

    pub fn freq_display(&self) -> String {
        let hz = self.frequency;
        let mhz = hz / 1_000_000;
        let khz = (hz % 1_000_000) / 1_000;
        let sub = hz % 1_000;
        format!("{mhz:3}.{khz:03}.{sub:03}")
    }

    pub fn smeter_label(&self) -> &'static str {
        match self.smeter {
            0..=2 => "S1",
            3..=5 => "S3",
            6..=8 => "S5",
            9..=11 => "S7",
            12..=15 => "S9",
            16..=20 => "S9+20",
            21..=25 => "S9+40",
            _ => "S9+60",
        }
    }

    pub fn filter_str(&self) -> String {
        format!("FIL-{}", self.filter + 1)
    }

    pub fn next_mode(&self) -> Mode {
        match self.mode {
            Some(Mode::Lsb) => Mode::Usb,
            Some(Mode::Usb) => Mode::Cw,
            Some(Mode::Cw) => Mode::CwR,
            Some(Mode::CwR) => Mode::Am,
            Some(Mode::Am) => Mode::Fm,
            Some(Mode::Fm) => Mode::Dig,
            Some(Mode::Dig) | None => Mode::Lsb,
        }
    }

    pub fn next_filter(&self) -> u8 {
        (self.filter + 1) % 4
    }

    pub fn voltage_display(&self) -> String {
        format!("{:.1}V", self.voltage as f32 / 10.0)
    }

    pub fn swr_display(&self) -> String {
        if !self.ptt || self.swr == 0 {
            "---".to_string()
        } else {
            format!("{}", self.swr)
        }
    }
}
