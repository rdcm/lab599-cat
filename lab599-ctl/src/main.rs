mod app;
mod args;
mod audio;
mod config;
mod events;
mod pages;
mod radio;
mod spectrum;
mod state;
mod ui;

use app::App;
use args::Args;
use audio::list_audio_devices;
use clap::Parser;
use config::Config;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.list_audio {
        list_audio_devices();
        return Ok(());
    }

    let config = Config::from_args(&args)?;
    App::new(config)?.run()
}
