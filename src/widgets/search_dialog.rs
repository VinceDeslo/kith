use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget},
    Frame
};

pub struct SearchDialog {
    pub search: String,
    cursor_index: usize,
}

impl SearchDialog {
    pub fn new() -> SearchDialog {
        return SearchDialog {
            search: String::new(), 
            cursor_index: 0,
        }
    }

    fn render_search(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(" Search ")
            .borders(Borders::ALL)
            .padding(Padding::new(1, 1, 1, 1));

        let input = Paragraph::new(self.search.clone()).block(block);

        Widget::render(Clear, area, buf);
        Widget::render(input, area, buf);
    }

    pub fn set_cursor(&self, frame: &mut Frame, area: Rect) {
        let reduced_area = get_search_dialog_layout(30, 10, area);

        // Increment positions by two due to padding on the paragraph block
        let x_position = reduced_area.x + self.cursor_index as u16 + 2;
        let y_position = reduced_area.y + 2;
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
        let byte_index = self.search
            .char_indices()
            .map(|(index, _)| index)
            .nth(self.cursor_index)
            .unwrap_or(self.search.len());

        self.search.insert(byte_index, character);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let current_index = self.cursor_index;
        let before_delete = self.search.chars().take(current_index - 1);
        let after_delete = self.search.chars().skip(current_index);

        self.search = before_delete.chain(after_delete).collect();
        self.move_cursor_left();
    }

    pub fn reset(&mut self) {
        self.search.clear();
        self.cursor_index = 0;
    }

    fn clamp_index(&self, index: usize) -> usize {
        let char_count = self.search.chars().count();
        return index.clamp(0, char_count)
    }
}

impl Widget for &SearchDialog {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let reduced_area = get_search_dialog_layout(30, 10, area);
        self.render_search(reduced_area, buf);
    }
}

pub fn get_search_dialog_layout(percent_x: u16, percent_y: u16, rectangle: Rect) -> Rect {
    let margin_x = (100 - percent_x) / 2;
    let margin_y = (100 - percent_y) / 2;

    let popup_layout = Layout::vertical([
        Constraint::Percentage(margin_y),
        Constraint::Percentage(percent_y),
        Constraint::Percentage(margin_y),
    ])
    .split(rectangle);

    Layout::horizontal([
        Constraint::Percentage(margin_x),
        Constraint::Percentage(percent_x),
        Constraint::Percentage(margin_x),
    ])
    .split(popup_layout[1])[1]
}
