use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct SettingsWidget {
    pub cursor: usize,
    // Audio Monitor (rows 0-1)
    pub audio_active: bool,
    pub audio_device: String,
    pub audio_device_editable: bool,
    // IQ Spectrum (rows 2-4)
    pub iq_active: bool,
    pub iq_device: String,
    pub iq_device_editable: bool,
    pub iq_rate: u32,
    // Remote (row 5)
    pub remote_active: bool,
    pub rx_socket: String,
    // CAT Connection (rows 6-8); serial port is read-only
    pub cat_serial_port: String,
    pub cat_baud: u32,
    pub cat_poll_ms: u64,
}

impl Widget for SettingsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer = Block::default().borders(Borders::ALL).title(" Settings ");
        let inner = outer.inner(area);
        outer.render(area, buf);

        let c = self.cursor;

        let focused = |pos: usize| -> Style {
            if c == pos {
                Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::new()
            }
        };
        let arrow = |pos: usize| -> Style {
            if c == pos {
                Style::new().fg(Color::Yellow)
            } else {
                Style::new().fg(Color::DarkGray)
            }
        };
        let refresh_s = |pos: usize| -> Style {
            if c == pos {
                Style::new().fg(Color::Cyan)
            } else {
                Style::new().fg(Color::DarkGray)
            }
        };

        let on_s = Style::new().fg(Color::Green).add_modifier(Modifier::BOLD);
        let off_s = Style::new().fg(Color::DarkGray);
        let toggle_spans = |active: bool| -> (&'static str, Style) {
            if active {
                ("[ ON  ]", on_s)
            } else {
                ("[ OFF ]", off_s)
            }
        };

        let selector_line = |label: &str,
                             value: &str,
                             editable: bool,
                             row: usize,
                             show_refresh: bool|
         -> Line<'static> {
            let mut spans = vec![Span::styled(format!("  {:<18}", label), focused(row))];
            if editable {
                spans.push(Span::styled("< ", arrow(row)));
                spans.push(Span::raw(value.to_string()));
                spans.push(Span::styled(" >", arrow(row)));
            } else {
                spans.push(Span::raw(value.to_string()));
            }
            if show_refresh {
                spans.push(Span::styled("   [R]", refresh_s(row)));
            }
            Line::from(spans)
        };

        let (audio_txt, audio_s) = toggle_spans(self.audio_active);
        let (iq_txt, iq_s) = toggle_spans(self.iq_active);
        let (remote_txt, remote_s) = toggle_spans(self.remote_active);

        let apply_s = if c == 8 {
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::new().fg(Color::DarkGray)
        };

        let lines: Vec<Line> = vec![
            Line::from(""),
            // ── Audio Monitor ──────────────────────────────────
            Line::from(vec![
                Span::styled("  Audio Monitor   ", focused(0)),
                Span::styled(audio_txt, audio_s),
            ]),
            selector_line(
                "Audio device",
                &self.audio_device,
                self.audio_device_editable,
                1,
                true,
            ),
            Line::from(""),
            // ── IQ Spectrum ────────────────────────────────────
            Line::from(vec![
                Span::styled("  IQ Spectrum     ", focused(2)),
                Span::styled(iq_txt, iq_s),
            ]),
            selector_line(
                "Audio device",
                &self.iq_device,
                self.iq_device_editable,
                3,
                true,
            ),
            Line::from(vec![
                Span::styled("  Sample rate     ", focused(4)),
                Span::styled("< ", arrow(4)),
                Span::raw(fmt_rate(self.iq_rate)),
                Span::styled(" >", arrow(4)),
            ]),
            Line::from(""),
            // ── Remote ─────────────────────────────────────────
            Line::from(vec![
                Span::styled("  Remote          ", focused(5)),
                Span::styled(remote_txt, remote_s),
            ]),
            Line::from(vec![Span::styled(
                format!("  RX socket        {}", self.rx_socket),
                Style::new().fg(Color::DarkGray),
            )]),
            Line::from(""),
            // ── Radio poll (immediate, no reconnect needed) ────
            selector_line("Radio poll", &fmt_poll(self.cat_poll_ms), true, 6, false),
            Line::from(""),
            // ── CAT Connection (baud change requires reconnect) ─
            Line::from(vec![Span::styled(
                format!("  Serial port      {}", self.cat_serial_port),
                Style::new().fg(Color::DarkGray),
            )]),
            selector_line("Baud rate", &self.cat_baud.to_string(), true, 7, false),
            Line::from(vec![
                Span::styled("  CAT Reconnect   ", focused(8)),
                Span::styled("[↵ Apply]", apply_s),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "  ↑↓ navigate   ←→ change   Enter toggle/apply   R refresh",
                Style::new().fg(Color::DarkGray),
            )]),
        ];

        Paragraph::new(lines).render(inner, buf);
    }
}

fn fmt_rate(hz: u32) -> String {
    match hz {
        44_100 => "44.1 kHz".to_string(),
        r if r % 1_000 == 0 => format!("{} kHz", r / 1_000),
        r => format!("{r} Hz"),
    }
}

fn fmt_poll(ms: u64) -> String {
    if ms >= 1_000 && ms.is_multiple_of(1_000) {
        format!("{} s", ms / 1_000)
    } else {
        format!("{ms} ms")
    }
}
