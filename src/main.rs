use std::io::Result;

mod tui;
mod app;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    app::App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
