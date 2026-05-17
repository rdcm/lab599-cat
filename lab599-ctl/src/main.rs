mod app;
mod app_state;
mod app_utils;
mod args;
mod config;
mod hardware;
mod input;
mod ui;
mod util;

use app::App;
use args::Args;
use clap::Parser;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config::from_args(&args)?;
    App::new(config).await?.run()
}
