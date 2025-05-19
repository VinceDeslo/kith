#![allow(unused)]
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Borders, Clear, Padding, Widget}};

use crate::{
    core::tsh::{self, ConnectionArgs, Database},
    widgets::{
        confirmation_toggle::{ConfirmationOption, ConfirmationToggle},
        database_name_input::{self, DatabaseNameInput},
        dialog::get_dialog_layout,
        user_list::StatefulUserList,
    }
};

pub enum Step {
    UserSelection,
    DatabaseInput,
    Confirmation,
}

pub struct ConnectDialog {
    pub user_list: StatefulUserList,
    pub database_name_input: DatabaseNameInput,
    pub confirmation_toggle: ConfirmationToggle,
    pub ready_to_connect: Option<bool>,
    pub selected_entry: Option<Database>,
    pub db_name: String,
    pub db_user: String,
    pub current_step: Step,

    cursor_index: usize,
}

impl Widget for &ConnectDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.current_step {
            Step::UserSelection => self.render_user_selection(area, buf),
            Step::DatabaseInput => self.render_db_name_input(area, buf),
            Step::Confirmation => self.render_confirmation(area, buf),
        } 
    }
}

impl ConnectDialog {
    pub fn new() -> ConnectDialog {
        return ConnectDialog {
            user_list: StatefulUserList::new(),
            database_name_input: DatabaseNameInput::new(),
            confirmation_toggle: ConfirmationToggle::new(),
            ready_to_connect: None,
            selected_entry: None,
            db_name: String::new(),
            db_user: String::new(),
            current_step: Step::UserSelection,
            cursor_index: 0,
        }
    }

    pub fn next_step(&mut self) {
        match self.current_step {
            Step::UserSelection => self.navigate_to_db_input(),
            Step::DatabaseInput => self.navigate_to_confirmation(), 
            Step::Confirmation => self.connect(),
        }
    }

    pub fn set_database_name_state(&mut self) {
        self.db_name = self.database_name_input.database_name.clone();
    }

    pub fn reset(&mut self) {
        self.user_list.reset();
        self.database_name_input.reset();
        self.confirmation_toggle.reset();

        self.current_step = Step::UserSelection;
        self.ready_to_connect = None;
        self.selected_entry = None;
        self.db_name.clear();
        self.db_user.clear();
        self.cursor_index = 0;
    }

    pub fn to_connection_args(&self) -> ConnectionArgs {
        return ConnectionArgs {
            instance: self.selected_entry.as_ref().unwrap().metadata.name.clone(),
            db_name: self.db_name.clone(),
            db_user: self.db_user.clone()
        }
    }

    fn navigate_to_db_input(&mut self) {
        match self.user_list.state.selected() {
            Some(index) => {
                let selected_user = &self.user_list.items[index];
                self.db_user = selected_user.to_string();
            },
            None => {},
        }

        if !self.db_user.is_empty() {
            self.current_step = Step::DatabaseInput;
        }
    }

    fn navigate_to_confirmation(&mut self) {
        if !self.db_name.is_empty() {
            self.current_step = Step::Confirmation;
        }
    }

    fn connect(&mut self) {
        match self.confirmation_toggle.get_selected() {
            ConfirmationOption::Yes => self.ready_to_connect = Some(true),
            ConfirmationOption::No => self.ready_to_connect = Some(false),
        }
    }

    fn render_user_selection(&self, area: Rect, buf: &mut Buffer) {
        let user_select_dialog_area = get_dialog_layout(30, 30, area);

        let block = Block::new()
            .title(" Select User ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1));

        Widget::render(Clear, user_select_dialog_area, buf);
        Widget::render(block, user_select_dialog_area, buf);

        self.user_list.render(user_select_dialog_area, buf);
    }

    fn render_db_name_input(&self, area: Rect, buf: &mut Buffer) {
        let database_input_dialog_area = get_dialog_layout(30, 10, area);

        let block = Block::new()
            .title(" Input Database Name ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1));

        Widget::render(Clear, database_input_dialog_area, buf);
        Widget::render(block, database_input_dialog_area, buf);

        self.database_name_input.render(database_input_dialog_area, buf);
    }

    fn render_confirmation(&self, area: Rect, buf: &mut Buffer) {
        let confirmation_dialog_area = get_dialog_layout(30, 15, area);

        let block = Block::new()
            .title(" Connect? ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1));

        Widget::render(Clear, confirmation_dialog_area, buf);
        Widget::render(block, confirmation_dialog_area, buf);

        self.confirmation_toggle.render(confirmation_dialog_area, buf);
    }
}
