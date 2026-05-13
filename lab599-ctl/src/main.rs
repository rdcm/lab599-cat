mod app;
mod args;
mod config;
mod hardware;
mod input;
mod ui;

use crate::hardware::audio::list_audio_devices;
use app::App;
use args::Args;
use clap::Parser;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.list_audio {
        list_audio_devices();
        return Ok(());
    }

    let config = Config::from_args(&args)?;
    App::new(config).await?.run()
}
