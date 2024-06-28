#![allow(unreachable_code)]
use std::io::Result;
use log::info;

mod tui;
mod app;
mod core;
mod widgets;

use core::tsh::Tsh;

fn main() -> Result<()> {
    env_logger::init();

    info!("Starting program");

    // Testing DS
    // let mut teleport = Tsh::new();
    // teleport.login("snyk.teleport.sh:443", "snyk.teleport.sh");
    // teleport.read_databases("native-pr-experience-polaris-prod-mt-us-1");

    let mut terminal = tui::init()?;
    app::App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
