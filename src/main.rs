#![allow(unreachable_code)]
use std::{fs::{create_dir_all, File}, io::Result, path::PathBuf};
use tracing::event;
use dotenv::dotenv;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod tui;
mod app;
mod config;
mod core;
mod widgets;

fn main() -> Result<()> {
    dotenv().ok();

    let log_dir = get_log_dir();
    create_dir_all(&log_dir)?;
    let log_path = log_dir.join("cli.log");

    let log_file = File::create(log_path)?;
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .init();

    let mut config = config::Config::new();
    config.load();

    event!(tracing::Level::INFO, "starting tui");

    let mut terminal = tui::init()?;

    let mut application = app::App::new(config);

    application.run(&mut terminal)?;

    tui::restore()?;

    if application.initiate_connection {
        application.connect_to_database();
    }

    Ok(())
}

fn get_log_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        // ~/Library/Logs/kith/...
        let home_dir = dirs::home_dir()
            .expect("failed to fetch home dir");
        home_dir.join("Library/Logs/kith")
    }
    #[cfg(target_os = "linux")]
    {
        // ~/.kith/...
        let home_dir = dirs::home_dir()
            .expect("failed to fetch home dir");
        home_dir.join(".kith")
    }
}
