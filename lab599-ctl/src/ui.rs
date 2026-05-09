use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline},
    Frame,
};

use std::sync::{Arc, Mutex};

use crate::spectrum::{SpectrumBins, FFT_SIZE};
use crate::state::RadioState;

pub fn draw(
    frame: &mut Frame,
    state: &RadioState,
    iq: Option<&(SpectrumBins, u32, Arc<Mutex<bool>>)>,
) {
    let has_spectrum = iq.is_some();
    let mut constraints = vec![
        Constraint::Length(9),
        Constraint::Length(4),
        Constraint::Length(4),
        Constraint::Length(6),
    ];
    if has_spectrum {
        constraints.push(Constraint::Length(10));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());

    render_radio_info(frame, state, chunks[0]);
    render_smeter(frame, state, chunks[1]);
    render_status(frame, state, chunks[2]);
    render_help(frame, chunks[3]);
    if let Some((bins, sample_rate, is_stereo)) = iq {
        let stereo = is_stereo.lock().map(|v| *v).unwrap_or(false);
        if let Ok(data) = bins.lock() {
            render_spectrum(frame, &data, *sample_rate, stereo, state.dc_suppress, chunks[4]);
        }
        render_error_log(frame, state, chunks[5]);
    } else {
        render_error_log(frame, state, chunks[4]);
    }
}

fn render_radio_info(frame: &mut Frame, state: &RadioState, area: Rect) {
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

    let title = Line::from(vec![
        Span::raw(" Lab599 "),
        Span::styled(
            state.model.clone(),
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
    ]);

    frame.render_widget(
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(title)),
        area,
    );
}

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

fn render_smeter(frame: &mut Frame, state: &RadioState, area: Rect) {
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

    let label = state.smeter_label();

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
                format!("  {label}"),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" S-Meter ")),
        area,
    );
}

fn render_status(frame: &mut Frame, state: &RadioState, area: Rect) {
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

fn render_help(frame: &mut Frame, area: Rect) {
    let lines = vec![
        Line::from("  ←/→  tune   ↑/↓  step   +/-  ±1 MHz   [/]  band"),
        Line::from("  m  mode    f  filter   p  preamp   a  att    s  split   t  TX"),
        Line::from("  c  cmr     v  VOX      n  NR        b  NB     x  notch  o  mon   d  DIF   z  DC∅"),
        Line::from("  q / Ctrl+C  quit"),
    ];

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Keys ")),
        area,
    );
}

fn suppress_lo_spike(data: &mut [u64]) {
    let inner_w = data.len();
    if inner_w <= 30 {
        return;
    }
    let c = inner_w / 2;
    let null_half = (inner_w / 24).max(3);
    let ref_l = c.saturating_sub(null_half + 8);
    let ref_r = (c + null_half + 8).min(inner_w - 1);

    let ctx_start = ref_l.saturating_sub(8);
    let noise_amp = if ref_l > ctx_start {
        let ctx = &data[ctx_start..ref_l];
        let mn = ctx.iter().cloned().min().unwrap_or(0);
        let mx = ctx.iter().cloned().max().unwrap_or(0);
        mx.saturating_sub(mn) as f64 * 0.75
    } else {
        3.0
    };

    let v_l = data[ref_l] as f64;
    let v_r = data[ref_r] as f64;
    let span = (ref_r - ref_l) as f64;
    let null_start = c.saturating_sub(null_half);
    let null_end = (c + null_half).min(inner_w - 1);

    for i in null_start..=null_end {
        let t = (i.saturating_sub(ref_l)) as f64 / span;
        let baseline = v_l + t * (v_r - v_l);
        let h = (i as u32).wrapping_mul(2654435761u32);
        let noise = (h as f64 / u32::MAX as f64 - 0.5) * noise_amp;
        data[i] = (baseline + noise).max(0.0) as u64;
    }
}

fn render_spectrum(
    frame: &mut Frame,
    bins: &[f32],
    sample_rate: u32,
    is_stereo: bool,
    dc_suppress: bool,
    area: Rect,
) {
    let inner_w = area.width.saturating_sub(2) as usize;
    if inner_w == 0 {
        return;
    }

    let (display_bins, bw_label) = if is_stereo {
        let bw_khz = sample_rate / 2 / 1000;
        (bins, format!("±{bw_khz} kHz"))
    } else {
        let bw_khz = sample_rate / 2 / 1000;
        (&bins[..FFT_SIZE / 2], format!("0\u{2013}{bw_khz} kHz"))
    };

    let dc_label = if dc_suppress { " DC∅" } else { "" };
    let title = if is_stereo {
        format!(" Spectrum {bw_label}{dc_label} ")
    } else {
        format!(" Audio {bw_label} ")
    };

    let n = display_bins.len();
    let mut data: Vec<u64> = (0..inner_w)
        .map(|col| {
            let lo = col * n / inner_w;
            let hi = ((col + 1) * n / inner_w).max(lo + 1).min(n);
            let max_db = display_bins[lo..hi]
                .iter()
                .cloned()
                .fold(f32::NEG_INFINITY, f32::max);
            // Map [-120, -20] dB → [0, 100]
            (max_db + 120.0).clamp(0.0, 100.0) as u64
        })
        .collect();

    if is_stereo && dc_suppress {
        suppress_lo_spike(&mut data);
    }

    frame.render_widget(
        Sparkline::default()
            .block(Block::default().borders(Borders::ALL).title(title))
            .data(&data)
            .max(100)
            .style(Style::default().fg(Color::Green)),
        area,
    );
}

fn render_error_log(frame: &mut Frame, state: &RadioState, area: Rect) {
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
