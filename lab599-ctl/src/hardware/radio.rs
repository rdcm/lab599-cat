use anyhow::Result;
use lab599_cat::{CatDriver, MeterType, Mode};
use serialport::SerialPort;

use crate::hardware::radio_state::{Model, RadioState};
use crate::hardware::serial::Serial;

pub struct Radio {
    device: CatDriver<Box<dyn SerialPort>>,
    state: RadioState,
    active_port: String,
    active_baud: u32,
}

impl Radio {
    pub async fn new(path: &str, baud: u32) -> Result<Self> {
        let mut device = CatDriver::new(Serial::open_port(path, baud).await?);
        let state = RadioState {
            model: device.get_id().map(Model::from).unwrap_or_default(),
            ..Default::default()
        };
        let active_port = Self::resolve_port(path);
        Ok(Self {
            device,
            state,
            active_port,
            active_baud: baud,
        })
    }

    pub fn active_port(&self) -> &str {
        &self.active_port
    }

    /// Reconnect to the current port with a new baud rate. Blocks for ~200 ms
    /// while the FTDI chip stabilises — call outside of terminal.draw().
    pub fn reconnect_blocking(&mut self, baud: u32) -> Result<()> {
        let port = Serial::open_port_blocking(&self.active_port, baud)?;
        self.device = CatDriver::new(port);
        self.active_baud = baud;
        self.state.model = self.device.get_id().map(Model::from).unwrap_or_default();
        Ok(())
    }

    fn resolve_port(path: &str) -> String {
        std::fs::canonicalize(path)
            .ok()
            .and_then(|p| p.into_os_string().into_string().ok())
            .unwrap_or_else(|| path.to_string())
    }

    pub fn state(&self) -> &RadioState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut RadioState {
        &mut self.state
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

    pub fn get_frequency(&mut self) -> Result<u64> {
        Ok(self.device.get_frequency_a()?)
    }

    pub fn get_mode(&mut self) -> Result<Option<Mode>> {
        Ok(Some(self.device.get_mode()?))
    }

    pub fn get_filter(&mut self) -> Result<u8> {
        let (rx, _) = self.device.get_filter()?;
        Ok(rx)
    }

    pub fn get_smeter(&mut self) -> Result<u16> {
        Ok(self.device.get_smeter()?)
    }

    pub fn get_ptt(&mut self) -> Result<bool> {
        Ok(self.device.get_ptt()?)
    }

    pub fn get_cmr(&mut self) -> Result<bool> {
        Ok(self.device.get_speech_compressor()?)
    }

    pub fn get_preamp(&mut self) -> Result<bool> {
        Ok(self.device.get_preamp()?)
    }

    pub fn get_attenuator(&mut self) -> Result<bool> {
        Ok(self.device.get_attenuator()?)
    }

    pub fn get_split(&mut self) -> Result<bool> {
        Ok(self.device.get_split()?)
    }

    pub fn get_vox(&mut self) -> Result<bool> {
        Ok(self.device.get_vox()?)
    }

    pub fn get_nr(&mut self) -> Result<bool> {
        Ok(self.device.get_noise_reduction()?)
    }

    pub fn get_nb(&mut self) -> Result<bool> {
        Ok(self.device.get_noise_blanker()?)
    }

    pub fn get_notch(&mut self) -> Result<bool> {
        Ok(self.device.get_notch()?)
    }

    pub fn get_mon(&mut self) -> Result<bool> {
        Ok(!self.device.get_monitor_mute()?)
    }

    pub fn get_dif(&mut self) -> Result<bool> {
        Ok(self.device.get_dsp_if()?)
    }

    pub fn get_power(&mut self) -> Result<u8> {
        Ok(self.device.get_power()?)
    }

    pub fn get_af_gain(&mut self) -> Result<u16> {
        Ok(self.device.get_af_gain()?)
    }

    pub fn get_voltage(&mut self) -> Result<u16> {
        Ok(self.device.get_voltage()?)
    }

    pub fn get_busy(&mut self) -> Result<bool> {
        Ok(self.device.get_busy()?)
    }

    pub fn get_swr(&mut self) -> Result<u16> {
        Ok(self.device.get_meter(MeterType::Swr)?)
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
