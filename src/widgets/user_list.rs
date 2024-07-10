use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget, Widget}
};
use crate::widgets::styles;

pub struct StatefulUserList {
    pub state: ListState,
    pub items: Vec<String>,
}

impl StatefulUserList {
    pub fn new() -> StatefulUserList {
        return StatefulUserList {
            state: ListState::default(),
            items: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.state = ListState::default();
        self.items = vec![];
    }

    pub fn with_items(&mut self, items: Vec<String>) {
        self.items = items;
    }

    fn render_user_entries(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .padding(Padding::new(5, 5, 2, 1));

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(_, item)| {
                ListItem::from(item.clone())
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(styles::SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state.clone());
    }
}

impl Widget for &StatefulUserList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_user_entries(area, buf);
    }
}
