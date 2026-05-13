use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::KeyCode;

use crate::app_state::AppState;
use crate::config::Config;
use crate::hardware::audio_builder::AudioBuilder;
use crate::hardware::radio::Radio;
use crate::hardware::serial::Serial;
use crate::input::keyboard::{Keyboard, Quit};
use crate::ui::layout::AppLayout;
use crate::ui::pages::page::Action;
use crate::ui::router::Router;

pub struct App {
    app_state: AppState,
    router: Router,
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
        .with_iq(config.iq_device.as_deref(), config.iq_rate)
        .build(|e| radio.log_error(e));

        let poll_interval = Duration::from_millis(config.poll_ms);

        Ok(Self {
            app_state: AppState {
                radio,
                audio,
                _config: config,
            },
            router: Router::new(),
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

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::TuneStep(dir) => {
                let delta = self.app_state.radio.state().step.hz() as i64 * dir as i64;
                self.app_state.radio.tune(delta);
            }
            Action::Tune(hz) => self.app_state.radio.tune(hz),
            Action::StepNext => self.app_state.radio.step_next(),
            Action::StepPrev => self.app_state.radio.step_prev(),
            Action::ToggleMode => self.app_state.radio.toggle_mode(),
            Action::ToggleFilter => self.app_state.radio.toggle_filter(),
            Action::TogglePtt => self.app_state.radio.toggle_ptt(),
            Action::TogglePreamp => self.app_state.radio.toggle_preamp(),
            Action::ToggleAttenuator => self.app_state.radio.toggle_attenuator(),
            Action::ToggleSplit => self.app_state.radio.toggle_split(),
            Action::ToggleCmr => self.app_state.radio.toggle_cmr(),
            Action::ToggleVox => self.app_state.radio.toggle_vox(),
            Action::ToggleNr => self.app_state.radio.toggle_nr(),
            Action::ToggleNb => self.app_state.radio.toggle_nb(),
            Action::ToggleNotch => self.app_state.radio.toggle_notch(),
            Action::ToggleMon => self.app_state.radio.toggle_mon(),
            Action::ToggleDif => self.app_state.radio.toggle_dif(),
            Action::ToggleDcSuppress => self.app_state.radio.toggle_dc_suppress(),
            Action::BandUp => self.app_state.radio.band_up(),
            Action::BandDown => self.app_state.radio.band_down(),
        }
    }

    fn event_loop(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        let mut last_poll = Instant::now();
        self.app_state.radio.tick();

        loop {
            let page_key = match Keyboard::read_key(50)? {
                Some(k) if k.code == KeyCode::Tab => {
                    self.router.advance_page();
                    None
                }
                key => key,
            };

            let mut action: Option<Action> = None;
            terminal.draw(|f| {
                let names = self.router.page_names();
                let inner = AppLayout::render(
                    f,
                    self.app_state.radio.state(),
                    &names,
                    self.router.current(),
                );
                if let Some(page) = self.router.current_page_mut() {
                    action = page.render(f, inner, &self.app_state, page_key);
                }
            })?;
            if let Some(a) = action {
                self.handle_action(a);
            }

            if last_poll.elapsed() >= self.poll_interval {
                self.app_state.radio.tick();
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
