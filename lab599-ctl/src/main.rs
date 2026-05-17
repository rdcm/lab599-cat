mod app;
mod app_config;
mod app_state;
mod app_utils;
mod args;
mod hardware;
mod input;
mod ui;

use app::App;
use app_config::AppConfig;
use args::Args;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = AppConfig::from_args(&args)?;
    App::new(config).await?.run()
}
