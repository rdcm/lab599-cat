use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::state::RadioState;

use super::page::Page;

pub struct LogsPage;

impl Page for LogsPage {
    fn name(&self) -> &str {
        "Logs"
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let items: Vec<ListItem> = if state.errors.is_empty() {
            vec![ListItem::new(Span::styled(
                " (no errors)",
                Style::default().fg(Color::DarkGray),
            ))]
        } else {
            state
                .errors
                .iter()
                .map(|(ts, msg)| {
                    let secs = ts.elapsed().as_secs();
                    let label = if secs < 60 {
                        format!(" [{secs:>3}s ago]  {msg}")
                    } else {
                        format!(" [{:>3}m ago]  {msg}", secs / 60)
                    };
                    ListItem::new(Span::styled(label, Style::default().fg(Color::Red)))
                })
                .collect()
        };

        frame.render_widget(
            List::new(items).block(Block::default().borders(Borders::ALL).title(" Error Log ")),
            area,
        );
    }
}
