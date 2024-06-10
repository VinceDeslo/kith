#![allow(unreachable_code)]
use std::io::Result;

use log::info;
use tsh::Tsh;

mod tui;
mod app;
mod tsh;

fn main() -> Result<()> {
    env_logger::init();

    info!("Starting program");

    // Testing DS
    let mut teleport = Tsh::new();
    teleport.login("snyk.teleport.sh:443", "snyk.teleport.sh");
    teleport.read_databases("native-pr-experience-polaris-prod-mt-us-1");

    return Ok(());

    let mut terminal = tui::init()?;
    app::App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
