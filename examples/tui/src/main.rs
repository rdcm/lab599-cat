mod app;
mod args;
mod audio;
mod radio;
mod state;
mod ui;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    if let Err(e) = app::run(&args) {
        ratatui::restore();
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
