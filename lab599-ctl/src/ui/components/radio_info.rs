use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{state::RadioState, ui::component::Component};

pub struct RadioInfo;

impl Component for RadioInfo {
    fn constraint(&self) -> Constraint {
        Constraint::Length(9)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let ptt_style = if state.ptt {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green)
        };

        let mut freq_spans = vec![
            Span::raw("  VFO A:  "),
            Span::styled(
                state.freq_display(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" MHz"),
        ];
        if state.audio_active {
            freq_spans.push(Span::styled(
                "  ● AUDIO",
                Style::default().fg(Color::Magenta),
            ));
        }

        let text = vec![
            Line::from(freq_spans),
            Line::from(vec![
                Span::raw("  Mode:   "),
                Span::styled(
                    state.mode_str(),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  Step:   "),
                Span::styled(state.step.label(), Style::default().fg(Color::Blue)),
            ]),
            Line::from(vec![
                Span::raw("  Filter: "),
                Span::styled(state.filter_str(), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  PTT:    "),
                Span::styled(if state.ptt { "TX" } else { "RX" }, ptt_style),
            ]),
            Line::from(vec![
                Span::raw("  Power:  "),
                Span::styled(
                    format!("{:3}%", state.power),
                    Style::default().fg(Color::White),
                ),
                Span::raw("     Volt: "),
                Span::styled(state.voltage_display(), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("  SWR:    "),
                Span::styled(state.swr_display(), Style::default().fg(Color::White)),
                Span::raw("      AF:  "),
                Span::styled(
                    format!("{}", state.af_gain),
                    Style::default().fg(Color::White),
                ),
            ]),
        ];

        frame.render_widget(
            Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(" Info ")),
            area,
        );
    }
}
