use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget}, 
    Frame
};

use crate::tui;
use crate::config::Config;
use crate::core::tsh::Tsh;
use crate::widgets::{
    connect_dialog::{ConnectDialog, Step},
    database_list::StatefulDatabaseList,
    search_dialog::SearchDialog
};

enum InputMode {
    Normal,
    Searching,
    Connecting,
}

pub struct App {
    pub initiate_connection: bool,
    teleport: Tsh,
    config: Config,
    database_list: StatefulDatabaseList,
    search_dialog: SearchDialog,
    connect_dialog: ConnectDialog,
    input_mode: InputMode,
    logged_in: bool,
    exit: bool,
    show_search: bool,
    show_connect: bool,
}

impl App {
    pub fn new(config: Config) -> App {
        App {
            teleport: Tsh::new(),
            config,
            database_list: StatefulDatabaseList::new(),
            search_dialog: SearchDialog::new(),
            connect_dialog: ConnectDialog::new(),
            input_mode: InputMode::Normal,
            logged_in: false,
            exit: false,
            show_search: false,
            show_connect: false,
            initiate_connection: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()>{
        while !self.exit {
            // Render new state
            terminal.draw(|frame| self.render_frame(frame))?;

            // Take input
            self.handle_events()?;

            // Update state
            self.set_database_list_state();
            self.set_selected_database_state();
            self.set_user_list_state();
            self.connect_dialog.set_database_name_state();
        }
        Ok(())
    }

    pub fn connect_to_database(&self) {
        let args = self.connect_dialog.to_connection_args();
        self.teleport.connect(args);
    }

    fn render_frame(&self, frame: &mut Frame){
        let frame_size = frame.size();
        frame.render_widget(self, frame_size);

        self.enable_cursor(frame, frame_size);
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
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('l') => self.handle_login(),
                KeyCode::Char('s') => self.toggle_search(),
                KeyCode::Char('c') => self.toggle_connect(),
                KeyCode::Down => self.handle_database_list_next(),
                KeyCode::Up => self.handle_database_list_previous(),
                _ => {},
            },
            InputMode::Searching => match key_event.code {
                KeyCode::Esc => self.exit_search(),
                KeyCode::Enter => self.handle_search(),
                KeyCode::Char(to_enter) => self.search_dialog.enter_char(to_enter),
                KeyCode::Backspace => self.search_dialog.delete_char(),
                KeyCode::Left => self.search_dialog.move_cursor_left(),
                KeyCode::Right => self.search_dialog.move_cursor_right(),
                _ => {},
            },
            InputMode::Connecting => match key_event.code {
                KeyCode::Esc => self.exit_connect(),
                KeyCode::Enter => self.handle_connect(),
                KeyCode::Down => self.handle_connect_down(),
                KeyCode::Up => self.handle_connect_up(),
                KeyCode::Char(to_enter) => self.handle_connect_char_input(to_enter),
                KeyCode::Backspace => self.handle_connect_backspace(),
                KeyCode::Left => self.handle_connect_left(),
                KeyCode::Right => self.handle_connect_right(),
                _ => {},
            },
        } 
    }

    fn handle_login(&mut self) {
        self.teleport.login(&self.config.tsh_proxy, &self.config.tsh_cluster);
        self.logged_in = true;
    }

    fn toggle_search(&mut self) {
        self.input_mode = InputMode::Searching;
        self.show_search = !self.show_search;
    }

    fn handle_search(&mut self) {
        self.handle_login();
        self.teleport.read_databases(&self.search_dialog.search);
        self.exit_search();
    }

    fn enable_cursor(&self, frame: &mut Frame, area: Rect) {
        let (_, main_area, _) = get_high_level_areas(area);

        match self.input_mode {
            InputMode::Searching => {
                self.search_dialog.set_cursor(frame, main_area);
            },
            InputMode::Connecting => {
                match self.connect_dialog.current_step {
                    Step::DatabaseInput => {
                        self.connect_dialog.database_name_input.set_cursor(frame, main_area);
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn exit_search(&mut self) {
        self.search_dialog.reset();
        self.input_mode = InputMode::Normal;
        self.show_search = false;
    }

    fn toggle_connect(&mut self) {
        self.input_mode = InputMode::Connecting;
        self.show_connect = !self.show_connect;
    }

    fn handle_connect(&mut self) {
        self.connect_dialog.next_step();
        match self.connect_dialog.ready_to_connect {
            Some(flag) => {
                match flag {
                    true => self.breakout_and_connect(),
                    false => self.exit_connect(),
                }
            },
            None => {},
        }
    }

    fn handle_connect_char_input(&mut self, to_enter: char) {
        match self.connect_dialog.current_step {
            Step::DatabaseInput => {
                self.connect_dialog.database_name_input.enter_char(to_enter);
            },
            _ => {},
        }
    }

    fn handle_connect_backspace(&mut self) {
        match self.connect_dialog.current_step {
            Step::DatabaseInput => {
                self.connect_dialog.database_name_input.delete_char();
            },
            _ => {},
        }
    }

    fn handle_connect_down(&mut self) {
        match self.connect_dialog.current_step {
            Step::UserSelection => {
                self.connect_dialog.user_list.state.select_next();
            },
            Step::Confirmation => {
                self.connect_dialog.confirmation_toggle.toggle();
            },
            _ => {},
        }
    }

    fn handle_connect_up(&mut self) {
        match self.connect_dialog.current_step {
            Step::UserSelection => {
                self.connect_dialog.user_list.state.select_previous();
            },
            Step::Confirmation => {
                self.connect_dialog.confirmation_toggle.toggle();
            },
            _ => {},
        }
    }

    fn handle_connect_left(&mut self) {
        match self.connect_dialog.current_step {
            Step::DatabaseInput => {
                self.connect_dialog.database_name_input.move_cursor_left();
            },
            _ => {},
        }
    }

    fn handle_connect_right(&mut self) {
        match self.connect_dialog.current_step {
            Step::DatabaseInput => {
                self.connect_dialog.database_name_input.move_cursor_right();
            },
            _ => {},
        }
    }

    fn exit_connect(&mut self) {
        self.connect_dialog.reset();
        self.input_mode = InputMode::Normal;
        self.show_connect = false;
    }

    fn set_database_list_state(&mut self) {
        self.database_list.with_items(self.teleport.databases.to_vec());
    }

    fn handle_database_list_next(&mut self) {
        self.database_list.state.select_next();
    }

    fn handle_database_list_previous(&mut self) {
        self.database_list.state.select_previous();
    }

    fn set_user_list_state(&mut self) {
        match &self.connect_dialog.selected_entry {
            Some(entry) => {
                self.connect_dialog.user_list.with_items(entry.users.allowed.clone());
            },
            None => {},
        }
    }

    fn set_selected_database_state(&mut self) {
        match &self.database_list.state.selected() {
            Some(index) => {
                let database = &self.database_list.items[*index];
                self.connect_dialog.selected_entry = Some(database.clone());
            },
            None => {},
        }
    }

    fn breakout_and_connect(&mut self) {
        self.initiate_connection = true;
        self.exit();
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (header_area, main_area, footer_area) = get_high_level_areas(area);

        render_header(header_area, buf);

        if !self.teleport.databases.is_empty() {
            self.database_list.render(main_area, buf);
        }
        if self.show_search {
            self.search_dialog.render(main_area, buf);
        }
        if self.show_connect {
            self.connect_dialog.render(main_area, buf);
        }

        render_footer(footer_area, buf)
    }
}

fn get_high_level_areas(area: Rect) -> (Rect, Rect, Rect) {
    let vertical = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(0),
        Constraint::Length(2),
    ]);
    let [header_area, main_area, footer_area] = vertical.areas(area);

    return (header_area, main_area, footer_area);
}

fn render_header(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Kith")
        .bold()
        .centered()
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Paragraph::new("\n<s> Search, ↓↑ Move, <c> Connect, <esc> Escape Dialog, <q> Quit")
        .centered()
        .render(area, buf);
}
