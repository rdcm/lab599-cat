use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use lab599_cat::CatDriver;

use crate::{
    args::Args,
    audio::{find_audio_device, list_audio_devices, start_audio},
    radio::{open_port, poll_radio},
    state::{RadioState, Step},
    ui::draw,
};

pub fn run(args: &Args) -> Result<()> {
    if args.list_audio {
        list_audio_devices();
        return Ok(());
    }

    let port_path = args
        .port
        .as_deref()
        .ok_or_else(|| anyhow::anyhow!("--port is required (e.g. --port /dev/ttyUSB0)"))?;

    let port = open_port(port_path, args.baud)?;
    let mut device = CatDriver::new(port);

    let _audio = args
        .audio
        .as_deref()
        .and_then(find_audio_device)
        .and_then(|d| start_audio(d).ok());

    let mut state = RadioState {
        audio_active: _audio.is_some(),
        step: Step::default(),
        model: device
            .get_id()
            .map(model_from_id)
            .unwrap_or_else(|_| "Unknown".into()),
        ..Default::default()
    };

    poll_radio(&mut device, &mut state);

    let mut terminal = ratatui::init();
    let poll_interval = Duration::from_millis(args.poll_ms);
    let mut last_poll = Instant::now();

    loop {
        terminal.draw(|f| draw(f, &state))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _) | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,

                    (KeyCode::Right, _) => {
                        let delta = state.step.hz() as i64;
                        tune(&mut device, &mut state, delta);
                    }
                    (KeyCode::Left, _) => {
                        let delta = state.step.hz() as i64;
                        tune(&mut device, &mut state, -delta);
                    }

                    (KeyCode::Up, _) => state.step = state.step.next(),
                    (KeyCode::Down, _) => state.step = state.step.prev(),

                    (KeyCode::PageUp, _) | (KeyCode::Char('+'), _) => {
                        tune(&mut device, &mut state, 1_000_000);
                    }
                    (KeyCode::PageDown, _) | (KeyCode::Char('-'), _) => {
                        tune(&mut device, &mut state, -1_000_000);
                    }

                    (KeyCode::Char('m'), _) => {
                        let next = state.next_mode();
                        match device.set_mode(next) {
                            Ok(()) => state.mode = Some(next),
                            Err(e) => state.log_error(format!("MD: {e}")),
                        }
                    }

                    (KeyCode::Char('f'), _) => {
                        let rx = state.next_filter();
                        let tx = rx.min(1);
                        match device.set_filter(rx, tx) {
                            Ok(()) => state.filter = rx,
                            Err(e) => state.log_error(format!("FL: {e}")),
                        }
                    }

                    (KeyCode::Char('t'), _) => {
                        let next = !state.ptt;
                        let result = if next {
                            device.set_tx()
                        } else {
                            device.set_rx()
                        };
                        match result {
                            Ok(()) => state.ptt = next,
                            Err(e) => state.log_error(format!("PT: {e}")),
                        }
                    }

                    (KeyCode::Char('p'), _) => {
                        let next = !state.preamp;
                        match device.set_preamp(next) {
                            Ok(()) => state.preamp = next,
                            Err(e) => state.log_error(format!("PA: {e}")),
                        }
                    }

                    (KeyCode::Char('a'), _) => {
                        let next = !state.attenuator;
                        match device.set_attenuator(next) {
                            Ok(()) => state.attenuator = next,
                            Err(e) => state.log_error(format!("RA: {e}")),
                        }
                    }

                    (KeyCode::Char('s'), _) => {
                        let next = !state.split;
                        match device.set_split(next) {
                            Ok(()) => state.split = next,
                            Err(e) => state.log_error(format!("SP: {e}")),
                        }
                    }

                    (KeyCode::Char('c'), _) => {
                        let next = !state.cmr;
                        match device.set_speech_compressor(next) {
                            Ok(()) => state.cmr = next,
                            Err(e) => state.log_error(format!("PR: {e}")),
                        }
                    }

                    (KeyCode::Char('v'), _) => {
                        let next = !state.vox;
                        match device.set_vox(next) {
                            Ok(()) => state.vox = next,
                            Err(e) => state.log_error(format!("VX: {e}")),
                        }
                    }

                    (KeyCode::Char('n'), _) => {
                        let next = !state.nr;
                        match device.set_noise_reduction(next) {
                            Ok(()) => state.nr = next,
                            Err(e) => state.log_error(format!("NR: {e}")),
                        }
                    }

                    (KeyCode::Char('b'), _) => {
                        let next = !state.nb;
                        match device.set_noise_blanker(next) {
                            Ok(()) => state.nb = next,
                            Err(e) => state.log_error(format!("NB: {e}")),
                        }
                    }

                    (KeyCode::Char('x'), _) => {
                        let next = !state.notch;
                        match device.set_notch(next) {
                            Ok(()) => state.notch = next,
                            Err(e) => state.log_error(format!("NT: {e}")),
                        }
                    }

                    (KeyCode::Char('o'), _) => {
                        let next = !state.mon;
                        match device.set_monitor_mute(!next) {
                            Ok(()) => state.mon = next,
                            Err(e) => state.log_error(format!("MO: {e}")),
                        }
                    }

                    (KeyCode::Char('d'), _) => {
                        let next = !state.dif;
                        match device.set_dsp_if(next) {
                            Ok(()) => state.dif = next,
                            Err(e) => state.log_error(format!("IS: {e}")),
                        }
                    }

                    (KeyCode::Char('['), _) => {
                        if let Err(e) = device.band_down() {
                            state.log_error(format!("BD: {e}"));
                        }
                    }

                    (KeyCode::Char(']'), _) => {
                        if let Err(e) = device.band_up() {
                            state.log_error(format!("BU: {e}"));
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
    Ok(())
}

fn model_from_id(id: u16) -> String {
    match id {
        500 => "TX-500".into(),
        505 => "TX-500MP".into(),
        n => format!("Lab599 (ID:{n})"),
    }
}

fn tune(
    device: &mut CatDriver<Box<dyn serialport::SerialPort>>,
    state: &mut RadioState,
    delta: i64,
) {
    let freq = (state.frequency as i64 + delta).max(0) as u64;
    match device.set_frequency_a(freq) {
        Ok(()) => state.frequency = freq,
        Err(e) => state.log_error(format!("FA: {e}")),
    }
}
