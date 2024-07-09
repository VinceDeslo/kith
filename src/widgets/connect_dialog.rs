#![allow(unused)]
use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Borders, Clear, Padding, Widget}};

use crate::{core::tsh::DatabaseEntry, widgets::dialog::get_dialog_layout};

use crate::widgets::user_list::StatefulUserList;

enum Step {
    UserSelection,
    DatabaseInput,
    Confirmation,
}

enum ConfirmationOptions {
    Yes,
    No,
}

impl ConfirmationOptions {
    fn to_string(&self) -> &str {
        match self {
            ConfirmationOptions::Yes => "Yes",
            ConfirmationOptions::No => "No",
        }
    }
}

pub struct ConnectDialog {
    pub user_list: StatefulUserList,
    pub ready_to_connect: bool,
    pub selected_entry: Option<DatabaseEntry>,
    pub db_name: String,
    pub db_user: String,

    current_step: Step,
    cursor_index: usize,
    confirmation: ConfirmationOptions,
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
            ready_to_connect: false,
            current_step: Step::UserSelection,
            selected_entry: None,
            db_name: String::new(),
            db_user: String::new(),
            cursor_index: 0,
            confirmation: ConfirmationOptions::No,
            user_list: StatefulUserList::new(),
        }
    }

    pub fn next_step(&mut self) {
        match self.current_step {
            Step::UserSelection => self.navigate_to_db_input(),
            Step::DatabaseInput => self.navigate_to_confirmation(), 
            Step::Confirmation => self.connect(),
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
        match self.confirmation {
            ConfirmationOptions::Yes => self.ready_to_connect = true, 
            ConfirmationOptions::No => self.ready_to_connect = false,
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
    }

    fn render_confirmation(&self, area: Rect, buf: &mut Buffer) {
        let confirmation_dialog_area = get_dialog_layout(30, 10, area);

        let block = Block::new()
            .title(" Connect? ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1));

        Widget::render(Clear, confirmation_dialog_area, buf);
        Widget::render(block, confirmation_dialog_area, buf);
    }

    pub fn reset(&mut self) {
        self.current_step = Step::UserSelection;
        self.db_name.clear();
        self.db_user.clear();
        self.cursor_index = 0;
    }
}
