use std::time::Duration;

use anyhow::{Context, Result};
use lab599_cat::{CatDriver, MeterType};
use serialport::SerialPort;

use crate::state::RadioState;

/// TX-500 uses FTDI FT232R (VID 0x0403, PID 0x6001).
/// Prefers stable /dev/serial/by-id/ symlinks; falls back to VID/PID scan.
pub fn auto_detect_port() -> Option<String> {
    if let Ok(entries) = std::fs::read_dir("/dev/serial/by-id") {
        for entry in entries.flatten() {
            if entry
                .file_name()
                .to_string_lossy()
                .to_lowercase()
                .contains("ftdi")
            {
                return entry.path().to_str().map(str::to_owned);
            }
        }
    }
    serialport::available_ports()
        .ok()?
        .into_iter()
        .find_map(|p| {
            if let serialport::SerialPortType::UsbPort(info) = p.port_type {
                if info.vid == 0x0403 && info.pid == 0x6001 {
                    return Some(p.port_name);
                }
            }
            None
        })
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
