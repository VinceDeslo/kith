use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Padding, Paragraph, Widget},
    Frame
};

use super::dialog::get_dialog_layout;

pub struct DatabaseNameInput {
    pub database_name: String,
    cursor_index: usize,
}

impl Widget for &DatabaseNameInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_database_name_input(area, buf);
    }
}

impl DatabaseNameInput {
    pub fn new() -> DatabaseNameInput {
        return DatabaseNameInput {
            database_name: String::new(), 
            cursor_index: 0,
        }
    }

    fn render_database_name_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .padding(Padding::new(2, 1, 2, 1));

        let input = Paragraph::new(self.database_name.clone())
            .block(block);

        Widget::render(input, area, buf);
    }

    pub fn set_cursor(&self, frame: &mut Frame, area: Rect) {
        let dialog_area = get_dialog_layout(30, 10, area);

        // Increment positions by two due to padding on the paragraph block
        let x_position = dialog_area.x + self.cursor_index as u16 + 2;
        let y_position = dialog_area.y + 2;
        frame.set_cursor(x_position, y_position);
    }

    pub fn move_cursor_right(&mut self) {
        let moved = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_index(moved);
    } 

    pub fn move_cursor_left(&mut self) {
        let moved = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_index(moved);
    }

    pub fn enter_char(&mut self, character: char) {
        let byte_index = self.database_name
            .char_indices()
            .map(|(index, _)| index)
            .nth(self.cursor_index)
            .unwrap_or(self.database_name.len());

        self.database_name.insert(byte_index, character);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let current_index = self.cursor_index;
        let before_delete = self.database_name.chars().take(current_index - 1);
        let after_delete = self.database_name.chars().skip(current_index);

        self.database_name = before_delete.chain(after_delete).collect();
        self.move_cursor_left();
    }

    pub fn reset(&mut self) {
        self.database_name.clear();
        self.cursor_index = 0;
    }

    fn clamp_index(&self, index: usize) -> usize {
        let char_count = self.database_name.chars().count();
        return index.clamp(0, char_count)
    }
}
