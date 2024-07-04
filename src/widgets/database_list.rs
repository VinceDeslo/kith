#![allow(unused)]
use std::borrow::Borrow;

use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::{palette::tailwind::SLATE, Modifier, Style, Stylize}, text::Line, widgets::{
        block::{Position, Title}, Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap
    }
};
use super::super::core::tsh::DatabaseEntry;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug, Default)]
pub struct StatefulDatabaseList {
    pub state: ListState,
    pub items: Vec<DatabaseEntry>,
}

impl StatefulDatabaseList {
    pub fn new() -> StatefulDatabaseList {
        return StatefulDatabaseList {
            state: ListState::default(),
            items: vec![],
        }
    }

    pub fn with_items(&mut self, items: Vec<DatabaseEntry>) {
        self.items = items;
    }

    fn render_database_entries(&self, area: Rect, buf: &mut Buffer) {
        let entry_count = self.items.len();
        let title = [" Databases ", "(", entry_count.to_string().as_str(), ") "].join("");

        let block = Block::new()
            .title(Line::raw(title).centered())
            .borders(Borders::ALL)
            .padding(Padding::new(5, 5, 1, 1));

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                ListItem::from(item.name.clone())
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state.clone());
    }

    fn render_database_details(&self, area: Rect, buf: &mut Buffer) {
        let selected = self.state.selected();
        match selected {
            Some(index) => {
                let entry = self.items[index].clone();

                let block = Block::new()
                    .title(Line::raw(" Details ").centered())
                    .borders(Borders::ALL)
                    .padding(Padding::new(5, 5, 1, 1));

                let details = entry.format_details();

                Paragraph::new(details)
                    .wrap(Wrap { trim: true })
                    .block(block)
                    .render(area, buf);
            },
            None => {},
        }
    }
}

impl Widget for &StatefulDatabaseList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]);
        let [left_area, right_area] = vertical.areas(area);

        self.render_database_entries(left_area, buf);
        self.render_database_details(right_area, buf);
    }
}
