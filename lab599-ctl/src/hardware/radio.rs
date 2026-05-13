use anyhow::Result;
use lab599_cat::{CatDriver, MeterType};
use serialport::SerialPort;

use crate::hardware::serial::Serial;
use crate::hardware::state::{Model, RadioState};

macro_rules! poll {
    ($state:expr, $call:expr, $field:expr, $tag:literal) => {
        match $call {
            Ok(v) => $field = v,
            Err(e) => $state.log_error(format!(concat!($tag, ": {}"), e)),
        }
    };
}

pub struct Radio {
    device: CatDriver<Box<dyn SerialPort>>,
    state: RadioState,
}

impl Radio {
    pub async fn new(path: &str, baud: u32, audio_active: bool) -> Result<Self> {
        let mut device = CatDriver::new(Serial::open_port(path, baud).await?);
        let state = RadioState {
            model: device.get_id().map(Model::from).unwrap_or_default(),
            audio_active,
            ..Default::default()
        };
        Ok(Self { device, state })
    }

    pub fn state(&self) -> &RadioState {
        &self.state
    }

    pub fn log_error(&mut self, msg: String) {
        self.state.log_error(msg);
    }

    pub fn step_next(&mut self) {
        self.state.step = self.state.step.next();
    }

    pub fn step_prev(&mut self) {
        self.state.step = self.state.step.prev();
    }

    pub fn toggle_dc_suppress(&mut self) {
        self.state.dc_suppress = !self.state.dc_suppress;
    }

    pub fn tick(&mut self) {
        let s = &mut self.state;
        poll!(s, self.device.get_frequency_a(), s.frequency, "FA");
        poll!(s, self.device.get_mode().map(Some), s.mode, "MD");
        poll!(
            s,
            self.device.get_filter().map(|(rx, _)| rx),
            s.filter,
            "FL"
        );
        poll!(s, self.device.get_smeter(), s.smeter, "SM");
        poll!(s, self.device.get_ptt(), s.ptt, "PT");
        poll!(s, self.device.get_speech_compressor(), s.cmr, "PR");
        poll!(s, self.device.get_preamp(), s.preamp, "PA");
        poll!(s, self.device.get_attenuator(), s.attenuator, "RA");
        poll!(s, self.device.get_split(), s.split, "SP");
        poll!(s, self.device.get_vox(), s.vox, "VX");
        poll!(s, self.device.get_noise_reduction(), s.nr, "NR");
        poll!(s, self.device.get_noise_blanker(), s.nb, "NB");
        poll!(s, self.device.get_notch(), s.notch, "NT");
        poll!(
            s,
            self.device.get_monitor_mute().map(|muted| !muted),
            s.mon,
            "MO"
        );
        poll!(s, self.device.get_dsp_if(), s.dif, "IS");
        poll!(s, self.device.get_power(), s.power, "PC");
        poll!(s, self.device.get_af_gain(), s.af_gain, "AG");
        poll!(s, self.device.get_voltage(), s.voltage, "VL");
        poll!(s, self.device.get_busy(), s.busy, "BY");

        if s.ptt {
            poll!(s, self.device.get_meter(MeterType::Swr), s.swr, "RM");
        } else {
            s.swr = 0;
        }
    }

    pub fn tune(&mut self, delta: i64) {
        let freq = (self.state.frequency as i64 + delta).max(0) as u64;
        match self.device.set_frequency_a(freq) {
            Ok(()) => self.state.frequency = freq,
            Err(e) => self.state.log_error(format!("FA: {e}")),
        }
    }

    pub fn toggle_mode(&mut self) {
        let next = self.state.next_mode();
        match self.device.set_mode(next) {
            Ok(()) => self.state.mode = Some(next),
            Err(e) => self.state.log_error(format!("MD: {e}")),
        }
    }

    pub fn toggle_filter(&mut self) {
        let rx = self.state.next_filter();
        match self.device.set_filter(rx, rx.min(1)) {
            Ok(()) => self.state.filter = rx,
            Err(e) => self.state.log_error(format!("FL: {e}")),
        }
    }

    pub fn toggle_ptt(&mut self) {
        let next = !self.state.ptt;
        let result = if next {
            self.device.set_tx()
        } else {
            self.device.set_rx()
        };
        match result {
            Ok(()) => self.state.ptt = next,
            Err(e) => self.state.log_error(format!("PT: {e}")),
        }
    }

    pub fn toggle_preamp(&mut self) {
        let next = !self.state.preamp;
        match self.device.set_preamp(next) {
            Ok(()) => self.state.preamp = next,
            Err(e) => self.state.log_error(format!("PA: {e}")),
        }
    }

    pub fn toggle_attenuator(&mut self) {
        let next = !self.state.attenuator;
        match self.device.set_attenuator(next) {
            Ok(()) => self.state.attenuator = next,
            Err(e) => self.state.log_error(format!("RA: {e}")),
        }
    }

    pub fn toggle_split(&mut self) {
        let next = !self.state.split;
        match self.device.set_split(next) {
            Ok(()) => self.state.split = next,
            Err(e) => self.state.log_error(format!("SP: {e}")),
        }
    }

    pub fn toggle_cmr(&mut self) {
        let next = !self.state.cmr;
        match self.device.set_speech_compressor(next) {
            Ok(()) => self.state.cmr = next,
            Err(e) => self.state.log_error(format!("PR: {e}")),
        }
    }

    pub fn toggle_vox(&mut self) {
        let next = !self.state.vox;
        match self.device.set_vox(next) {
            Ok(()) => self.state.vox = next,
            Err(e) => self.state.log_error(format!("VX: {e}")),
        }
    }

    pub fn toggle_nr(&mut self) {
        let next = !self.state.nr;
        match self.device.set_noise_reduction(next) {
            Ok(()) => self.state.nr = next,
            Err(e) => self.state.log_error(format!("NR: {e}")),
        }
    }

    pub fn toggle_nb(&mut self) {
        let next = !self.state.nb;
        match self.device.set_noise_blanker(next) {
            Ok(()) => self.state.nb = next,
            Err(e) => self.state.log_error(format!("NB: {e}")),
        }
    }

    pub fn toggle_notch(&mut self) {
        let next = !self.state.notch;
        match self.device.set_notch(next) {
            Ok(()) => self.state.notch = next,
            Err(e) => self.state.log_error(format!("NT: {e}")),
        }
    }

    pub fn toggle_mon(&mut self) {
        let next = !self.state.mon;
        match self.device.set_monitor_mute(!next) {
            Ok(()) => self.state.mon = next,
            Err(e) => self.state.log_error(format!("MO: {e}")),
        }
    }

    pub fn toggle_dif(&mut self) {
        let next = !self.state.dif;
        match self.device.set_dsp_if(next) {
            Ok(()) => self.state.dif = next,
            Err(e) => self.state.log_error(format!("IS: {e}")),
        }
    }

    pub fn band_up(&mut self) {
        if let Err(e) = self.device.band_up() {
            self.state.log_error(format!("BU: {e}"));
        }
    }

    pub fn band_down(&mut self) {
        if let Err(e) = self.device.band_down() {
            self.state.log_error(format!("BD: {e}"));
        }
    }
}
