use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct StatusFlagsWidget {
    pub(crate) preamp: bool,
    pub(crate) attenuator: bool,
    pub(crate) split: bool,
    pub(crate) cmr: bool,
    pub(crate) vox: bool,
    pub(crate) mon: bool,
    pub(crate) nr: bool,
    pub(crate) nb: bool,
    pub(crate) notch: bool,
    pub(crate) dif: bool,
    pub(crate) busy: bool,
}

impl Widget for StatusFlagsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let flag = |on: bool, label: &'static str| -> Span<'static> {
            if on {
                Span::styled(format!(" [{label}]"), Style::default().fg(Color::Yellow))
            } else {
                Span::styled(format!("  {label} "), Style::default().fg(Color::DarkGray))
            }
        };

        let busy_span = if self.busy {
            Span::styled(
                " [BUSY]",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled("  BUSY ", Style::default().fg(Color::DarkGray))
        };

        let lines = vec![
            Line::from(vec![
                flag(self.preamp, "PRE"),
                flag(self.attenuator, "ATT"),
                flag(self.split, "SPL"),
                flag(self.cmr, "CMR"),
                flag(self.vox, "VOX"),
                flag(self.mon, "MON"),
            ]),
            Line::from(vec![
                flag(self.nr, "NR "),
                flag(self.nb, "NB "),
                flag(self.notch, "NF "),
                flag(self.dif, "DIF"),
                busy_span,
            ]),
        ];

        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" DSP & Modes "),
            )
            .render(area, buf);
    }
}
