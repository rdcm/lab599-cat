use std::path::PathBuf;

use anyhow::Result;

use crate::args::Args;

pub struct Config {
    pub port: Option<String>,
    pub baud: u32,
    pub audio_device: Option<String>,
    pub rx_socket: PathBuf,
    pub iq_device: Option<String>,
    pub iq_rate: u32,
    pub poll_ms: u64,
}

impl Config {
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(Self {
            port: args.port.clone(),
            baud: args.baud,
            audio_device: args.audio.clone(),
            rx_socket: args.rx_socket.clone(),
            iq_device: args.iq_device.clone(),
            iq_rate: args.iq_rate,
            poll_ms: args.poll_ms,
        })
    }
}
