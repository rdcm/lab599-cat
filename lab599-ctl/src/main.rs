mod app;
mod app_state;
mod app_utils;
mod args;
mod config;
mod hardware;
mod input;
mod ui;

use crate::hardware::audio::Audio;
use app::App;
use args::Args;
use clap::Parser;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.list_audio {
        Audio::list_devices();
        return Ok(());
    }

    let config = Config::from_args(&args)?;
    App::new(config).await?.run()
}
