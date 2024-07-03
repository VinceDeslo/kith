#![allow(unreachable_code)]
use std::io::Result;
use log::debug;

mod tui;
mod app;
mod core;
mod widgets;

fn main() -> Result<()> {
    env_logger::init();

    debug!("Starting program");

    let mut terminal = tui::init()?;
    app::App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
