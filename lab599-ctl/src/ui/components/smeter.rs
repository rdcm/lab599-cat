use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{state::RadioState, ui::component::Component};

const SMETER_TICKS: &[(&str, usize)] = &[
    ("1", 3),
    ("3", 6),
    ("5", 9),
    ("7", 12),
    ("9", 15),
    ("+20", 20),
    ("+40", 25),
    ("+60", 30),
];

pub struct Smeter;

impl Component for Smeter {
    fn constraint(&self) -> Constraint {
        Constraint::Length(4)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let inner_w = area.width.saturating_sub(2) as usize;
        if inner_w < 8 {
            return;
        }
        let max = 30usize;
        let val = (state.smeter as usize).min(max);

        let mut scale: Vec<char> = vec![' '; inner_w];
        for (label, tick_val) in SMETER_TICKS {
            let pos = tick_val * inner_w / max;
            for (i, c) in label.chars().enumerate() {
                if pos + i < inner_w {
                    scale[pos + i] = c;
                }
            }
        }

        let filled = val * inner_w / max;
        let green_end = 15 * inner_w / max;
        let yellow_end = 20 * inner_w / max;

        let green_fill = filled.min(green_end);
        let yellow_fill = filled.saturating_sub(green_end).min(yellow_end - green_end);
        let red_fill = filled.saturating_sub(yellow_end);
        let empty = inner_w - filled;

        let lines = vec![
            Line::from(Span::styled(
                scale.iter().collect::<String>(),
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(vec![
                Span::styled("█".repeat(green_fill), Style::default().fg(Color::Green)),
                Span::styled("█".repeat(yellow_fill), Style::default().fg(Color::Yellow)),
                Span::styled("█".repeat(red_fill), Style::default().fg(Color::Red)),
                Span::styled("░".repeat(empty), Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("  {}", state.smeter_label()),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        frame.render_widget(
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" S-Meter ")),
            area,
        );
    }
}
