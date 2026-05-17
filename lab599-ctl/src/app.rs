use std::time::{Duration, Instant};

use anyhow::Result;

use crate::app_state::AppState;
use crate::app_utils;
use crate::config::Config;
use crate::hardware::audio_builder::AudioBuilder;
use crate::hardware::radio::Radio;
use crate::hardware::serial::Serial;
use crate::input::keyboard::{Keyboard, Quit};
use crate::ui::components::spectrum::component::SpectrumComponent;
use crate::ui::layout::AppLayout;

pub struct App {
    app_state: AppState,
    layout: AppLayout,
    poll_interval: Duration,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let path = config
            .port
            .clone()
            .map_or_else(Serial::auto_detect_port, Ok)?;

        let mut radio = Radio::new(&path, config.baud, config.audio_device.is_some()).await?;

        let audio = if let Some(name) = config.audio_device.as_deref() {
            AudioBuilder::new().with_remote(name, &config.rx_socket)
        } else {
            AudioBuilder::new().with_loopback()
        }
        .build(|e| radio.log_error(e));

        // Initialize spectrum before ratatui::init() so that any CPAL/ALSA
        // messages written to stderr don't corrupt the TUI display.
        let spectrum = if let Some(device) = config.iq_device.as_deref() {
            match SpectrumComponent::start(device, config.iq_rate, audio.errors().clone()) {
                Ok(s) => s,
                Err(e) => {
                    radio.log_error(format!("Spectrum: {e}"));
                    SpectrumComponent::inactive()
                }
            }
        } else {
            SpectrumComponent::inactive()
        };

        let poll_interval = Duration::from_millis(config.poll_ms);

        Ok(Self {
            app_state: AppState {
                radio,
                audio,
                _config: config,
            },
            layout: AppLayout::new(spectrum),
            poll_interval,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        let result = self.event_loop(&mut terminal);
        ratatui::restore();
        match result {
            Err(e) if e.is::<Quit>() => Ok(()),
            other => other,
        }
    }

    fn event_loop(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        let mut last_poll = Instant::now();
        app_utils::tick(&mut self.app_state.radio);

        loop {
            let page_key = Keyboard::read_key(50)?;

            if page_key.is_some() || self.layout.needs_draw(&self.app_state) {
                terminal.draw(|f| {
                    self.layout.render(f, &mut self.app_state, page_key);
                })?;
            }

            if last_poll.elapsed() >= self.poll_interval {
                app_utils::tick(&mut self.app_state.radio);
                last_poll = Instant::now();
                if let Ok(mut errs) = self.app_state.audio.errors().lock() {
                    for e in errs.drain(..) {
                        self.app_state.radio.log_error(format!("Audio: {e}"));
                    }
                }
            }
        }
    }
}
