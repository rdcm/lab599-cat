use ratatui::{
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{state::RadioState, ui::component::Component};

use super::helpers::entry;

pub struct TuiHelp;

impl Component for TuiHelp {
    fn constraint(&self) -> Constraint {
        Constraint::Percentage(50)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, _state: &RadioState) {
        let lines = vec![
            entry("Tab      ", "Switch to next page"),
            entry("q        ", "Quit"),
            entry("Ctrl+C   ", "Quit"),
            Line::from(""),
            entry("z        ", "Toggle DC spike suppression"),
        ];

        frame.render_widget(
            Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title(" Navigation ")),
            area,
        );
    }
}
