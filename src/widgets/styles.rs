use ratatui::style::{palette::tailwind::SLATE, Modifier, Style};

pub const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
