use std::time::Duration;

use anyhow::{Context, Result};
use serialport::SerialPort;

pub struct Serial;

impl Serial {
    /// Blocking variant of open_port — identical behaviour but uses std::thread::sleep
    /// instead of tokio::time::sleep. Safe to call from a sync context.
    pub fn open_port_blocking(path: &str, baud: u32) -> Result<Box<dyn SerialPort>> {
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

    /// TX-500 uses FTDI FT232R (VID 0x0403, PID 0x6001).
    /// Prefers stable /dev/serial/by-id/ symlinks; falls back to VID/PID scan.
    pub fn auto_detect_port() -> Result<String> {
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

    pub async fn open_port(path: &str, baud: u32) -> Result<Box<dyn SerialPort>> {
        let port = serialport::new(path, baud)
            .timeout(Duration::from_millis(2000))
            .open()
            .with_context(|| format!("Cannot open serial port {path}"))?;

        port.clear(serialport::ClearBuffer::All)
            .with_context(|| "Cannot clear serial port buffer")?;

        tokio::time::sleep(Duration::from_millis(200)).await;
        port.clear(serialport::ClearBuffer::Input)?;

        Ok(port)
    }
}
