use std::time::Duration;

use anyhow::{Context, Result};
use lab599_cat::{CatDriver, CatError, MeterType};
use serialport::SerialPort;

use crate::hardware::state::RadioState;
use crate::ui::pages::page::Action;

pub struct Radio {
    device: CatDriver<Box<dyn SerialPort>>,
}

impl Radio {
    pub fn new(device: CatDriver<Box<dyn SerialPort>>) -> Self {
        Self { device }
    }

    pub fn get_id(&mut self) -> Result<u16, CatError> {
        self.device.get_id()
    }

    pub fn tick(&mut self, state: &mut RadioState) {
        poll_radio(&mut self.device, state);
    }

    pub fn handle(&mut self, action: Action, state: &mut RadioState) {
        match action {
            Action::TuneStep(dir) => {
                let delta = state.step.hz() as i64 * dir as i64;
                self.tune(state, delta);
            }
            Action::Tune(hz) => self.tune(state, hz),
            Action::StepNext => state.step = state.step.next(),
            Action::StepPrev => state.step = state.step.prev(),
            Action::ToggleMode => self.toggle_mode(state),
            Action::ToggleFilter => self.toggle_filter(state),
            Action::TogglePtt => self.toggle_ptt(state),
            Action::TogglePreamp => self.toggle_preamp(state),
            Action::ToggleAttenuator => self.toggle_attenuator(state),
            Action::ToggleSplit => self.toggle_split(state),
            Action::ToggleCmr => self.toggle_cmr(state),
            Action::ToggleVox => self.toggle_vox(state),
            Action::ToggleNr => self.toggle_nr(state),
            Action::ToggleNb => self.toggle_nb(state),
            Action::ToggleNotch => self.toggle_notch(state),
            Action::ToggleMon => self.toggle_mon(state),
            Action::ToggleDif => self.toggle_dif(state),
            Action::ToggleDcSuppress => state.dc_suppress = !state.dc_suppress,
            Action::BandUp => {
                if let Err(e) = self.device.band_up() {
                    state.log_error(format!("BU: {e}"));
                }
            }
            Action::BandDown => {
                if let Err(e) = self.device.band_down() {
                    state.log_error(format!("BD: {e}"));
                }
            }
        }
    }

    fn tune(&mut self, state: &mut RadioState, delta: i64) {
        let freq = (state.frequency as i64 + delta).max(0) as u64;
        match self.device.set_frequency_a(freq) {
            Ok(()) => state.frequency = freq,
            Err(e) => state.log_error(format!("FA: {e}")),
        }
    }

    fn toggle_mode(&mut self, state: &mut RadioState) {
        let next = state.next_mode();
        match self.device.set_mode(next) {
            Ok(()) => state.mode = Some(next),
            Err(e) => state.log_error(format!("MD: {e}")),
        }
    }

    fn toggle_filter(&mut self, state: &mut RadioState) {
        let rx = state.next_filter();
        match self.device.set_filter(rx, rx.min(1)) {
            Ok(()) => state.filter = rx,
            Err(e) => state.log_error(format!("FL: {e}")),
        }
    }

    fn toggle_ptt(&mut self, state: &mut RadioState) {
        let next = !state.ptt;
        let result = if next {
            self.device.set_tx()
        } else {
            self.device.set_rx()
        };
        match result {
            Ok(()) => state.ptt = next,
            Err(e) => state.log_error(format!("PT: {e}")),
        }
    }

    fn toggle_preamp(&mut self, state: &mut RadioState) {
        let next = !state.preamp;
        match self.device.set_preamp(next) {
            Ok(()) => state.preamp = next,
            Err(e) => state.log_error(format!("PA: {e}")),
        }
    }

    fn toggle_attenuator(&mut self, state: &mut RadioState) {
        let next = !state.attenuator;
        match self.device.set_attenuator(next) {
            Ok(()) => state.attenuator = next,
            Err(e) => state.log_error(format!("RA: {e}")),
        }
    }

    fn toggle_split(&mut self, state: &mut RadioState) {
        let next = !state.split;
        match self.device.set_split(next) {
            Ok(()) => state.split = next,
            Err(e) => state.log_error(format!("SP: {e}")),
        }
    }

    fn toggle_cmr(&mut self, state: &mut RadioState) {
        let next = !state.cmr;
        match self.device.set_speech_compressor(next) {
            Ok(()) => state.cmr = next,
            Err(e) => state.log_error(format!("PR: {e}")),
        }
    }

    fn toggle_vox(&mut self, state: &mut RadioState) {
        let next = !state.vox;
        match self.device.set_vox(next) {
            Ok(()) => state.vox = next,
            Err(e) => state.log_error(format!("VX: {e}")),
        }
    }

    fn toggle_nr(&mut self, state: &mut RadioState) {
        let next = !state.nr;
        match self.device.set_noise_reduction(next) {
            Ok(()) => state.nr = next,
            Err(e) => state.log_error(format!("NR: {e}")),
        }
    }

    fn toggle_nb(&mut self, state: &mut RadioState) {
        let next = !state.nb;
        match self.device.set_noise_blanker(next) {
            Ok(()) => state.nb = next,
            Err(e) => state.log_error(format!("NB: {e}")),
        }
    }

    fn toggle_notch(&mut self, state: &mut RadioState) {
        let next = !state.notch;
        match self.device.set_notch(next) {
            Ok(()) => state.notch = next,
            Err(e) => state.log_error(format!("NT: {e}")),
        }
    }

    fn toggle_mon(&mut self, state: &mut RadioState) {
        let next = !state.mon;
        match self.device.set_monitor_mute(!next) {
            Ok(()) => state.mon = next,
            Err(e) => state.log_error(format!("MO: {e}")),
        }
    }

    fn toggle_dif(&mut self, state: &mut RadioState) {
        let next = !state.dif;
        match self.device.set_dsp_if(next) {
            Ok(()) => state.dif = next,
            Err(e) => state.log_error(format!("IS: {e}")),
        }
    }
}

/// TX-500 uses FTDI FT232R (VID 0x0403, PID 0x6001).
/// Prefers stable /dev/serial/by-id/ symlinks; falls back to VID/PID scan.
pub fn auto_detect_port() -> anyhow::Result<String> {
    if let Ok(entries) = std::fs::read_dir("/dev/serial/by-id") {
        for entry in entries.flatten() {
            if entry
                .file_name()
                .to_string_lossy()
                .to_lowercase()
                .contains("ftdi")
            {
                if let Some(path) = entry.path().to_str().map(str::to_owned) {
                    return Ok(path);
                }
            }
        }
    }
    serialport::available_ports()
        .unwrap_or_default()
        .into_iter()
        .find_map(|p| {
            if let serialport::SerialPortType::UsbPort(info) = p.port_type {
                if info.vid == 0x0403 && info.pid == 0x6001 {
                    return Some(p.port_name);
                }
            }
            None
        })
        .ok_or_else(|| anyhow::anyhow!("TX-500 not found — use --port to specify manually"))
}

pub fn open_port(path: &str, baud: u32) -> Result<Box<dyn SerialPort>> {
    let port = serialport::new(path, baud)
        .timeout(Duration::from_millis(2000))
        .open()
        .with_context(|| format!("Cannot open serial port {path}"))?;

    port.clear(serialport::ClearBuffer::All)
        .with_context(|| "Cannot clear serial port buffer")?;

    std::thread::sleep(Duration::from_millis(200));
    port.clear(serialport::ClearBuffer::Input)?;

    Ok(port)
}

pub fn poll_radio(device: &mut CatDriver<Box<dyn SerialPort>>, state: &mut RadioState) {
    macro_rules! poll {
        ($call:expr, $field:expr, $tag:literal) => {
            match $call {
                Ok(v) => $field = v,
                Err(e) => state.log_error(format!(concat!($tag, ": {}"), e)),
            }
        };
    }

    poll!(device.get_frequency_a(), state.frequency, "FA");
    poll!(device.get_mode().map(Some), state.mode, "MD");
    poll!(device.get_filter().map(|(rx, _)| rx), state.filter, "FL");
    poll!(device.get_smeter(), state.smeter, "SM");
    poll!(device.get_ptt(), state.ptt, "PT");
    poll!(device.get_speech_compressor(), state.cmr, "PR");
    poll!(device.get_preamp(), state.preamp, "PA");
    poll!(device.get_attenuator(), state.attenuator, "RA");
    poll!(device.get_split(), state.split, "SP");
    poll!(device.get_vox(), state.vox, "VX");
    poll!(device.get_noise_reduction(), state.nr, "NR");
    poll!(device.get_noise_blanker(), state.nb, "NB");
    poll!(device.get_notch(), state.notch, "NT");
    poll!(
        device.get_monitor_mute().map(|muted| !muted),
        state.mon,
        "MO"
    );
    poll!(device.get_dsp_if(), state.dif, "IS");
    poll!(device.get_power(), state.power, "PC");
    poll!(device.get_af_gain(), state.af_gain, "AG");
    poll!(device.get_voltage(), state.voltage, "VL");
    poll!(device.get_busy(), state.busy, "BY");

    if state.ptt {
        poll!(device.get_meter(MeterType::Swr), state.swr, "RM");
    } else {
        state.swr = 0;
    }
}
