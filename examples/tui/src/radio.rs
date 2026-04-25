use std::time::Duration;

use anyhow::{Context, Result};
use lab599_cat_device::Tx500;
use serialport::SerialPort;

use crate::state::RadioState;

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

pub fn poll_radio(device: &mut Tx500<Box<dyn SerialPort>>, state: &mut RadioState) {
    match device.get_frequency_a() {
        Ok(f) => state.frequency = f,
        Err(e) => state.log_error(format!("FA: {e}")),
    }
    match device.get_mode() {
        Ok(m) => state.mode = Some(m),
        Err(e) => state.log_error(format!("MD: {e}")),
    }
    match device.get_filter() {
        Ok((rx, _)) => state.filter = rx,
        Err(e) => state.log_error(format!("FL: {e}")),
    }
    match device.get_smeter() {
        Ok(s) => state.smeter = s,
        Err(e) => state.log_error(format!("SM: {e}")),
    }
    match device.get_ptt() {
        Ok(p) => state.ptt = p,
        Err(e) => state.log_error(format!("PT: {e}")),
    }
    match device.get_speech_compressor() {
        Ok(v) => state.cmr = v,
        Err(e) => state.log_error(format!("PR: {e}")),
    }
}
