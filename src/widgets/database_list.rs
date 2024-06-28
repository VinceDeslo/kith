#![allow(unused)]
use ratatui::{
    buffer::Buffer, 
    layout::{Alignment, Constraint, Layout, Rect}, 
    text::Line, 
    widgets::{
        block::{Position, Title}, 
        Block, Borders, ListState, Paragraph, StatefulWidget, Widget
    },
    style::{Style, Stylize},
};
use super::super::core::tsh::DatabaseEntry;

#[derive(Debug, Default)]
pub struct StatefulDatabaseList {
    pub state: ListState,
    pub items: Vec<DatabaseEntry>,
}

impl StatefulDatabaseList {
    pub fn with_items(&mut self, items: Vec<DatabaseEntry>) {
        self.items = items;
    }

    pub fn next(&mut self) {
        !todo!()
    }

    pub fn previous(&mut self) {
        !todo!()
    }

    fn render_database_entries(&self, area: Rect, buf: &mut Buffer) {
        !todo!()
    }

    fn render_database_details(&self, area: Rect, buf: &mut Buffer) {
        !todo!()
    }
}

impl Widget for &StatefulDatabaseList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);
        let [upper_area, lower_area] = vertical.areas(area);

        // self.render_database_entries(upper_area, buf);
        // self.render_database_details(lower_area, buf);
    }
}
