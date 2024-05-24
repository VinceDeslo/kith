use std::io;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use ratatui::{buffer::Buffer, layout::{Alignment, Rect}, style::Stylize, symbols::border, text::Line, widgets::{block::{Position, Title}, Block, Borders, Paragraph, Widget}, Frame};

use super::tui;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()>{
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame){
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()>{
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        } 
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
            let title = Title::from(" Kith ");
            let commands = Title::from(Line::from(vec![
                " Quit ".into(),
                "<Q> ".bold(),
            ]));
            let block = Block::default()
                .title(title.alignment(Alignment::Center))
                .title(
                    commands
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .borders(Borders::ALL)
                .border_set(border::ROUNDED);

            Paragraph::new("Welcome to your DB connections!")
                .centered()
                .block(block)
                .white()
                .on_black()
                .render(area, buf);
    }
}
