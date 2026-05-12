use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::hardware::state::RadioState;

pub struct StatusFlagsWidget {
    preamp: bool,
    attenuator: bool,
    split: bool,
    cmr: bool,
    vox: bool,
    mon: bool,
    nr: bool,
    nb: bool,
    notch: bool,
    dif: bool,
    busy: bool,
}

impl From<&RadioState> for StatusFlagsWidget {
    fn from(s: &RadioState) -> Self {
        Self {
            preamp: s.preamp,
            attenuator: s.attenuator,
            split: s.split,
            cmr: s.cmr,
            vox: s.vox,
            mon: s.mon,
            nr: s.nr,
            nb: s.nb,
            notch: s.notch,
            dif: s.dif,
            busy: s.busy,
        }
    }
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
