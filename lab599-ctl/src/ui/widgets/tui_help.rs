use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::ui::ui_utils::entry;

pub struct TuiHelpWidget {
    pub(crate) dc_suppress: bool,
}

impl Widget for TuiHelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dc_label = format!(
            "Toggle DC spike suppression [{}]",
            if self.dc_suppress { "ON" } else { "OFF" }
        );

        let lines = vec![
            entry("Tab      ", "Switch to next page"),
            entry("q        ", "Quit"),
            entry("Ctrl+C   ", "Quit"),
            Line::from(""),
            entry("z        ", &dc_label),
        ];

        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" Navigation "))
            .render(area, buf);
    }
}
