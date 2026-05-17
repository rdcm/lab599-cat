use std::path::PathBuf;

use anyhow::Result;

use crate::args::Args;

pub struct Config {
    pub port: Option<String>,
    pub baud: u32,
    pub rx_socket: PathBuf,
    pub poll_ms: u64,
}

impl Config {
    pub fn from_args(args: &Args) -> Result<Self> {
        Ok(Self {
            port: args.port.clone(),
            baud: args.baud,
            rx_socket: args.rx_socket.clone(),
            poll_ms: args.poll_ms,
        })
    }
}
