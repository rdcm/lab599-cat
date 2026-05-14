use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct RadioInfoWidget {
    pub(crate) freq: String,
    pub(crate) mode: &'static str,
    pub(crate) step: &'static str,
    pub(crate) filter: String,
    pub(crate) ptt: bool,
    pub(crate) power: u8,
    pub(crate) voltage: String,
    pub(crate) swr: String,
    pub(crate) af_gain: u16,
    pub(crate) audio: bool,
}

impl Widget for RadioInfoWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let ptt_style = if self.ptt {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Green)
        };

        let mut freq_spans = vec![
            Span::raw("  VFO A:  "),
            Span::styled(
                self.freq,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" MHz"),
        ];
        if self.audio {
            freq_spans.push(Span::styled(
                "  ● AUDIO",
                Style::default().fg(Color::Magenta),
            ));
        }

        let lines = vec![
            Line::from(freq_spans),
            Line::from(vec![
                Span::raw("  Mode:   "),
                Span::styled(
                    self.mode,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::raw("  Step:   "),
                Span::styled(self.step, Style::default().fg(Color::Blue)),
            ]),
            Line::from(vec![
                Span::raw("  Filter: "),
                Span::styled(self.filter, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::raw("  PTT:    "),
                Span::styled(if self.ptt { "TX" } else { "RX" }, ptt_style),
            ]),
            Line::from(vec![
                Span::raw("  Power:  "),
                Span::styled(
                    format!("{:3}%", self.power),
                    Style::default().fg(Color::White),
                ),
                Span::raw("     Volt: "),
                Span::styled(self.voltage, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::raw("  SWR:    "),
                Span::styled(self.swr, Style::default().fg(Color::White)),
                Span::raw("      AF:  "),
                Span::styled(
                    format!("{}", self.af_gain),
                    Style::default().fg(Color::White),
                ),
            ]),
        ];

        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(" Info "))
            .render(area, buf);
    }
}
