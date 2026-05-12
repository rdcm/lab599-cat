use crate::hardware::state::RadioState;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Widget},
};

pub struct ErrorLogWidget {
    entries: Vec<String>,
}

impl From<&RadioState> for ErrorLogWidget {
    fn from(s: &RadioState) -> Self {
        let entries = s
            .errors
            .iter()
            .map(|(ts, msg)| {
                let secs = ts.elapsed().as_secs();
                if secs < 60 {
                    format!(" [{secs:>3}s ago]  {msg}")
                } else {
                    format!(" [{:>3}m ago]  {msg}", secs / 60)
                }
            })
            .collect();
        Self { entries }
    }
}

impl Widget for ErrorLogWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = if self.entries.is_empty() {
            vec![ListItem::new(Span::styled(
                " (no errors)",
                Style::default().fg(Color::DarkGray),
            ))]
        } else {
            self.entries
                .into_iter()
                .map(|e| ListItem::new(Span::styled(e, Style::default().fg(Color::Red))))
                .collect()
        };

        List::new(items)
            .block(Block::default().borders(Borders::ALL).title(" Error Log "))
            .render(area, buf);
    }
}
