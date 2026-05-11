use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{state::RadioState, ui::component::Component};

pub struct StatusFlags;

impl Component for StatusFlags {
    fn constraint(&self) -> Constraint {
        Constraint::Length(4)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let flag = |on: bool, label: &'static str| -> Span<'static> {
            if on {
                Span::styled(format!(" [{label}]"), Style::default().fg(Color::Yellow))
            } else {
                Span::styled(format!("  {label} "), Style::default().fg(Color::DarkGray))
            }
        };

        let busy_span = if state.busy {
            Span::styled(
                " [BUSY]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("  BUSY ", Style::default().fg(Color::DarkGray))
        };

        let lines = vec![
            Line::from(vec![
                flag(state.preamp, "PRE"),
                flag(state.attenuator, "ATT"),
                flag(state.split, "SPL"),
                flag(state.cmr, "CMR"),
                flag(state.vox, "VOX"),
                flag(state.mon, "MON"),
            ]),
            Line::from(vec![
                flag(state.nr, "NR "),
                flag(state.nb, "NB "),
                flag(state.notch, "NF "),
                flag(state.dif, "DIF"),
                busy_span,
            ]),
        ];

        frame.render_widget(
            Paragraph::new(lines).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" DSP & Modes "),
            ),
            area,
        );
    }
}
