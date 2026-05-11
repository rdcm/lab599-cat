use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub const KEY: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);

pub fn entry<'a>(key: &'a str, desc: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::raw("  "),
        Span::styled(key, KEY),
        Span::raw(desc),
    ])
}
