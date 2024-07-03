#![allow(unreachable_code)]
use std::io::Result;
use log::debug;

mod tui;
mod app;
mod config;
mod core;
mod widgets;

fn main() -> Result<()> {
    env_logger::init();

    let mut config = config::Config::new();
    config.load();

    debug!("Starting program");

    let mut terminal = tui::init()?;
    app::App::new(config).run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
