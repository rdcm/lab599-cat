use clap::Parser;

#[derive(Parser)]
#[command(name = "lab599-cat-tui", about = "Lab599 TX-500 control panel")]
pub struct Args {
    /// Serial port for CAT control (e.g. /dev/ttyUSB0)
    #[arg(short, long)]
    pub port: Option<String>,

    /// Serial port baud rate
    #[arg(short, long, default_value = "9600")]
    pub baud: u32,

    /// Poll interval for CAT status in milliseconds
    #[arg(long, default_value = "200")]
    pub poll_ms: u64,

    /// Unix socket path for RX audio streaming
    #[arg(long, default_value = "/tmp/lab599-rx.sock")]
    pub rx_socket: std::path::PathBuf,
}
