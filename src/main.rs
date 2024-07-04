#![allow(unreachable_code)]
use std::io::Result;
use log::debug;
use dotenv::dotenv;

mod tui;
mod app;
mod config;
mod core;
mod widgets;

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut config = config::Config::new();
    config.load();

    debug!("Starting program");

    let mut terminal = tui::init()?;

    let mut application = app::App::new(config);

    application.run(&mut terminal)?;

    tui::restore()?;

    Ok(())
}
