use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    widgets::{
        Block, HighlightSpacing, List, 
        ListItem, ListState, Padding, 
        StatefulWidget, Widget
    }
};

use crate::widgets::styles;

pub enum ConfirmationOption {
    Yes,
    No,
}

impl ConfirmationOption {
    pub fn to_index(&self) -> usize {
        match self {
            ConfirmationOption::Yes => 0,
            ConfirmationOption::No => 1,
        }
    }
    fn to_string(&self) -> &str {
        match self {
            ConfirmationOption::Yes => "Yes",
            ConfirmationOption::No => "No",
        }
    }
}

pub struct ConfirmationToggle {
    pub state: ListState,
    items: Vec<ConfirmationOption>,
}

impl Widget for &ConfirmationToggle {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_confirmation_toggle(area, buf);
    }
}

impl ConfirmationToggle {
    pub fn new() -> ConfirmationToggle {
        let mut toggle = ConfirmationToggle {
            state: ListState::default(),
            items: vec![
                ConfirmationOption::Yes,
                ConfirmationOption::No,
            ],
        };
        toggle.state.select(
                Some(ConfirmationOption::Yes.to_index())
        );
        return toggle
    }

    pub fn reset(&mut self) {
        self.state = ListState::default();
        self.state.select(
                Some(ConfirmationOption::Yes.to_index())
        );
    }

    pub fn toggle(&mut self) {
        match self.state.selected() {
            Some(index) => {
                match self.items[index] {
                    ConfirmationOption::Yes => self.state.select(
                        Some(ConfirmationOption::No.to_index())
                    ),
                    ConfirmationOption::No => self.state.select(
                        Some(ConfirmationOption::Yes.to_index())
                    ),
                }
            },
            None => {},
        }
    }

    pub fn get_selected(&self) -> &ConfirmationOption {
        match self.state.selected() {
            Some(index) => {
                match self.items.get(index) {
                    Some(value) => value,
                    None => &ConfirmationOption::No,
                }
            }
            None => &ConfirmationOption::No,
        }
    }

    pub fn set_state_to_default(&mut self) {

    }

    fn render_confirmation_toggle(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .padding(Padding::new(5, 5, 2, 1));

        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(_, item)| {
                ListItem::from(item.to_string())
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
