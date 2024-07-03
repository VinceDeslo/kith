use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget}, 
    Frame
};
use super::tui;
use super::config::Config;
use super::widgets::database_list::StatefulDatabaseList;
use super::core::tsh::Tsh;

#[derive(Debug, Default)]
pub struct App {
    teleport: Tsh,
    config: Config,
    database_list: StatefulDatabaseList,
    logged_in: bool,
    exit: bool,
}

impl App {
    pub fn new(config: Config) -> App {
        return App {
            teleport: Tsh::new(),
            config,
            database_list: StatefulDatabaseList::default(),
            logged_in: false,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()>{
        while !self.exit {
            // Render new state
            terminal.draw(|frame| self.render_frame(frame))?;

            // Take input
            self.handle_events()?;

            // Update state
            self.database_list.with_items(self.teleport.entries.to_vec())
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
            KeyCode::Char('l') => self.handle_login(),
            KeyCode::Char('s') => self.handle_search(),
            KeyCode::Down => self.handle_next(),
            KeyCode::Up => self.handle_previous(),
            _ => {}
        } 
    }

    fn handle_login(&mut self) {
        self.teleport.login(&self.config.tsh_proxy, &self.config.tsh_cluster);
        self.logged_in = true;
    }

    fn handle_search(&mut self) {
        self.handle_login();
        self.teleport.read_databases("native-pr-experience");
    }

    fn handle_next(&mut self) {
        self.database_list.state.select_next()
    }
    
    fn handle_previous(&mut self) {
        self.database_list.state.select_previous()
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ]);
        let [header_area, main_area, footer_area] = vertical.areas(area);

        render_header(header_area, buf);

        // let mut db_list = StatefulDatabaseList::default();

        if self.teleport.entries.len() != 0 {
            self.database_list.render(main_area, buf);
        }

        render_footer(footer_area, buf)
    }
}

fn render_header(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Kith")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\n<L> to login, <S> to search, ↓↑ to move, <Q> to quit")
        .centered()
        .render(area, buf);
}
