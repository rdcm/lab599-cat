use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use clap::Parser;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lab599_cat_core::Mode;
use lab599_cat_device::Tx500;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use serialport::SerialPort;

#[derive(Parser)]
#[command(name = "lab599-cat-tui", about = "Lab599 TX-500 control panel")]
struct Args {
    /// Serial port for CAT control (e.g. /dev/ttyUSB0)
    #[arg(short, long)]
    port: String,

    /// Serial port baud rate
    #[arg(short, long, default_value = "38400")]
    baud: u32,

    /// Audio input device name (substring match). Leave empty to list devices.
    #[arg(short, long, default_value = "")]
    audio: String,

    /// Poll interval for CAT status in milliseconds
    #[arg(long, default_value = "200")]
    poll_ms: u64,
}

#[derive(Clone, Default)]
struct RadioState {
    frequency: u64,
    mode: Option<Mode>,
    smeter: u16,
    ptt: bool,
    preamp: bool,
    attenuator: bool,
    split: bool,
    audio_active: bool,
    error: Option<String>,
}

impl RadioState {
    fn mode_str(&self) -> &'static str {
        match self.mode {
            Some(Mode::Lsb) => "LSB",
            Some(Mode::Usb) => "USB",
            Some(Mode::Cw) => "CW",
            Some(Mode::CwR) => "CW-R",
            Some(Mode::Fm) => "FM",
            Some(Mode::Am) => "AM",
            Some(Mode::Dig) => "DIG",
            None => "---",
        }
    }

    fn freq_display(&self) -> String {
        let hz = self.frequency;
        let mhz = hz / 1_000_000;
        let khz = (hz % 1_000_000) / 1_000;
        let sub = hz % 1_000;
        format!("{mhz:3}.{khz:03}.{sub:03}")
    }

    fn smeter_label(&self) -> String {
        let dots = self.smeter;
        let s = match dots {
            0..=3 => "S1",
            4..=6 => "S3",
            7..=9 => "S5",
            10..=12 => "S7",
            13..=15 => "S9",
            16..=20 => "S9+10",
            21..=25 => "S9+20",
            _ => "S9+40",
        };
        format!("{s} ({dots})")
    }
}

fn open_port(path: &str, baud: u32) -> Result<Box<dyn SerialPort>> {
    serialport::new(path, baud)
        .timeout(Duration::from_millis(500))
        .open()
        .with_context(|| format!("Cannot open serial port {path}"))
}

fn list_audio_devices() {
    let host = cpal::default_host();
    println!("Available audio input devices:");
    if let Ok(devices) = host.input_devices() {
        for d in devices {
            if let Ok(name) = d.name() {
                println!("  {name}");
            }
        }
    }
}

fn find_audio_device(name_pattern: &str) -> Option<cpal::Device> {
    let host = cpal::default_host();
    if name_pattern.is_empty() {
        return host.default_input_device();
    }
    host.input_devices().ok()?.find(|d| {
        d.name()
            .map(|n| n.to_lowercase().contains(&name_pattern.to_lowercase()))
            .unwrap_or(false)
    })
}

fn start_audio(device: cpal::Device) -> Result<cpal::Stream> {
    let config = device.default_input_config()?;
    let err_fn = |e| eprintln!("audio error: {e}");

    let stream = match config.sample_format() {
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            |data: &[f32], _| {
                // In a real app: send to output device or buffer for processing.
                // Here we just consume the data to keep the stream alive.
                let _ = data.len();
            },
            err_fn,
            None,
        )?,
        SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            |data: &[i16], _| {
                let _ = data.len();
            },
            err_fn,
            None,
        )?,
        SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            |data: &[u16], _| {
                let _ = data.len();
            },
            err_fn,
            None,
        )?,
        _ => anyhow::bail!("unsupported sample format"),
    };

    stream.play()?;
    Ok(stream)
}

fn poll_radio(device: &mut Tx500<Box<dyn SerialPort>>, state: &mut RadioState) {
    match device.get_frequency_a() {
        Ok(f) => state.frequency = f,
        Err(e) => state.error = Some(e.to_string()),
    }
    if let Ok(m) = device.get_mode() {
        state.mode = Some(m);
    }
    if let Ok(s) = device.get_smeter() {
        state.smeter = s;
    }
    if let Ok(p) = device.get_ptt() {
        state.ptt = p;
    }
}

fn draw(frame: &mut Frame, state: &RadioState) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(5),  // frequency + mode
            Constraint::Length(3),  // S-meter
            Constraint::Length(3),  // status flags
            Constraint::Min(4),     // help / log
        ])
        .split(area);

    // ── Frequency + Mode ──────────────────────────────────────────────────
    let ptt_style = if state.ptt {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };

    let freq_text = vec![
        Line::from(vec![
            Span::raw("  VFO A: "),
            Span::styled(
                state.freq_display(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" MHz"),
        ]),
        Line::from(vec![
            Span::raw("  Mode:  "),
            Span::styled(
                state.mode_str(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("  PTT:   "),
            Span::styled(
                if state.ptt { "TX" } else { "RX" },
                ptt_style,
            ),
            if state.audio_active {
                Span::styled("  ● AUDIO", Style::default().fg(Color::Magenta))
            } else {
                Span::raw("")
            },
        ]),
    ];

    let freq_block = Paragraph::new(freq_text)
        .block(Block::default().borders(Borders::ALL).title(" Lab599 TX-500 "));
    frame.render_widget(freq_block, chunks[0]);

    // ── S-meter ───────────────────────────────────────────────────────────
    let smeter_ratio = (state.smeter as f64 / 30.0).min(1.0);
    let smeter_color = match state.smeter {
        0..=12 => Color::Green,
        13..=20 => Color::Yellow,
        _ => Color::Red,
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(" S-Meter "))
        .gauge_style(Style::default().fg(smeter_color))
        .ratio(smeter_ratio)
        .label(state.smeter_label());
    frame.render_widget(gauge, chunks[1]);

    // ── Status flags ──────────────────────────────────────────────────────
    let flags: Vec<ListItem> = vec![
        format!(
            " Pre-amp: {}   Attenuator: {}   Split: {}",
            if state.preamp { "ON " } else { "off" },
            if state.attenuator { "ON " } else { "off" },
            if state.split { "ON " } else { "off" },
        ),
    ]
    .into_iter()
    .map(|s| ListItem::new(s))
    .collect();

    let status = List::new(flags)
        .block(Block::default().borders(Borders::ALL).title(" Status "));
    frame.render_widget(status, chunks[2]);

    // ── Help ──────────────────────────────────────────────────────────────
    let help_lines: Vec<Line> = vec![
        Line::from(" ↑/↓ : ±10 Hz     ←/→ : ±100 Hz     PgUp/PgDn : ±1 kHz"),
        Line::from(" +/- : ±1 MHz      m : cycle mode     t : toggle TX"),
        Line::from(" p : toggle pre-amp    a : toggle att    s : toggle split"),
        Line::from(" q / Ctrl+C : quit"),
        if let Some(err) = &state.error {
            Line::from(Span::styled(
                format!(" ERR: {err}"),
                Style::default().fg(Color::Red),
            ))
        } else {
            Line::from("")
        },
    ];

    let help = Paragraph::new(help_lines)
        .block(Block::default().borders(Borders::ALL).title(" Keys "));
    frame.render_widget(help, chunks[3]);
}

fn cycle_mode(current: Option<Mode>) -> Mode {
    match current {
        Some(Mode::Lsb) => Mode::Usb,
        Some(Mode::Usb) => Mode::Cw,
        Some(Mode::Cw) => Mode::CwR,
        Some(Mode::CwR) => Mode::Fm,
        Some(Mode::Fm) => Mode::Am,
        Some(Mode::Am) => Mode::Dig,
        Some(Mode::Dig) | None => Mode::Lsb,
    }
}

fn run(args: &Args) -> Result<()> {
    if args.audio.is_empty() {
        list_audio_devices();
        println!("\nUse --audio <pattern> to select a device, then --port <port> to connect.");
        return Ok(());
    }

    let port = open_port(&args.port, args.baud)?;
    let mut device = Tx500::new(port);

    let audio_device = find_audio_device(&args.audio);
    let _audio_stream = audio_device.map(|d| {
        start_audio(d).ok()
    }).flatten();

    let mut state = RadioState {
        audio_active: _audio_stream.is_some(),
        ..Default::default()
    };

    // Initial poll
    poll_radio(&mut device, &mut state);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = ratatui::init();

    let poll_interval = Duration::from_millis(args.poll_ms);
    let mut last_poll = Instant::now();

    loop {
        terminal.draw(|f| draw(f, &state))?;
        state.error = None;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _)
                    | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,

                    (KeyCode::Up, _) => {
                        let f = state.frequency.saturating_add(10);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::Down, _) => {
                        let f = state.frequency.saturating_sub(10);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::Right, _) => {
                        let f = state.frequency.saturating_add(100);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::Left, _) => {
                        let f = state.frequency.saturating_sub(100);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::PageUp, _) => {
                        let f = state.frequency.saturating_add(1_000);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::PageDown, _) => {
                        let f = state.frequency.saturating_sub(1_000);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::Char('+'), _) => {
                        let f = state.frequency.saturating_add(1_000_000);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::Char('-'), _) => {
                        let f = state.frequency.saturating_sub(1_000_000);
                        if let Err(e) = device.set_frequency_a(f) {
                            state.error = Some(e.to_string());
                        } else {
                            state.frequency = f;
                        }
                    }
                    (KeyCode::Char('m'), _) => {
                        let next = cycle_mode(state.mode);
                        if let Err(e) = device.set_mode(next) {
                            state.error = Some(e.to_string());
                        } else {
                            state.mode = Some(next);
                        }
                    }
                    (KeyCode::Char('t'), _) => {
                        let next = !state.ptt;
                        let result = if next {
                            device.set_tx()
                        } else {
                            device.set_rx()
                        };
                        if let Err(e) = result {
                            state.error = Some(e.to_string());
                        } else {
                            state.ptt = next;
                        }
                    }
                    (KeyCode::Char('p'), _) => {
                        let next = !state.preamp;
                        if let Err(e) = device.set_preamp(next) {
                            state.error = Some(e.to_string());
                        } else {
                            state.preamp = next;
                        }
                    }
                    (KeyCode::Char('a'), _) => {
                        let next = !state.attenuator;
                        if let Err(e) = device.set_attenuator(next) {
                            state.error = Some(e.to_string());
                        } else {
                            state.attenuator = next;
                        }
                    }
                    (KeyCode::Char('s'), _) => {
                        let next = !state.split;
                        if let Err(e) = device.set_split(next) {
                            state.error = Some(e.to_string());
                        } else {
                            state.split = next;
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_poll.elapsed() >= poll_interval {
            poll_radio(&mut device, &mut state);
            last_poll = Instant::now();
        }
    }

    ratatui::restore();
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
