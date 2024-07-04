use ratatui::layout::{Constraint, Layout, Rect};

pub fn get_dialog_layout(percent_x: u16, percent_y: u16, rectangle: Rect) -> Rect {
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
