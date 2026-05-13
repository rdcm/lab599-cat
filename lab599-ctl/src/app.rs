use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::KeyCode;

use crate::hardware::audio::Audio;
use crate::hardware::radio::Radio;
use crate::hardware::serial::Serial;
use crate::input::keyboard::{Keyboard, Quit};
use crate::ui::layout::AppLayout;
use crate::ui::pages::page::Action;
use crate::ui::router::Router;
use crate::{config::Config, hardware::spectrum::IqCapture};

pub struct App {
    radio: Radio,
    router: Router,
    _audio: Option<Audio>,
    _iq: Option<IqCapture>,
    poll_interval: Duration,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let audio = config
            .audio_device
            .as_deref()
            .and_then(|name| Audio::new(name, &config.rx_socket));

        let path = config.port.map_or_else(Serial::auto_detect_port, Ok)?;

        let mut radio = Radio::new(&path, config.baud, audio.is_some()).await?;

        let iq = config.iq_device.as_deref().and_then(|name| {
            IqCapture::new(name, config.iq_rate)
                .map_err(|e| radio.log_error(format!("IQ: {e}")))
                .ok()
        });

        Ok(Self {
            router: Router::new(iq.as_ref()),
            poll_interval: Duration::from_millis(config.poll_ms),
            _audio: audio,
            _iq: iq,
            radio,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let mut terminal = ratatui::init();
        self.radio.tick();
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
                let delta = self.radio.state().step.hz() as i64 * dir as i64;
                self.radio.tune(delta);
            }
            Action::Tune(hz) => self.radio.tune(hz),
            Action::StepNext => self.radio.step_next(),
            Action::StepPrev => self.radio.step_prev(),
            Action::ToggleMode => self.radio.toggle_mode(),
            Action::ToggleFilter => self.radio.toggle_filter(),
            Action::TogglePtt => self.radio.toggle_ptt(),
            Action::TogglePreamp => self.radio.toggle_preamp(),
            Action::ToggleAttenuator => self.radio.toggle_attenuator(),
            Action::ToggleSplit => self.radio.toggle_split(),
            Action::ToggleCmr => self.radio.toggle_cmr(),
            Action::ToggleVox => self.radio.toggle_vox(),
            Action::ToggleNr => self.radio.toggle_nr(),
            Action::ToggleNb => self.radio.toggle_nb(),
            Action::ToggleNotch => self.radio.toggle_notch(),
            Action::ToggleMon => self.radio.toggle_mon(),
            Action::ToggleDif => self.radio.toggle_dif(),
            Action::ToggleDcSuppress => self.radio.toggle_dc_suppress(),
            Action::BandUp => self.radio.band_up(),
            Action::BandDown => self.radio.band_down(),
        }
    }

    fn event_loop(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        let mut last_poll = Instant::now();

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
                let inner = AppLayout::render(f, self.radio.state(), &names, self.router.current());
                if let Some(page) = self.router.current_page_mut() {
                    action = page.render(f, inner, self.radio.state(), page_key);
                }
            })?;
            if let Some(a) = action {
                self.handle_action(a);
            }

            if last_poll.elapsed() >= self.poll_interval {
                self.radio.tick();
                last_poll = Instant::now();
                if let Some(iq) = &self._iq {
                    if let Ok(mut errs) = iq.errors.lock() {
                        for e in errs.drain(..) {
                            self.radio.log_error(format!("IQ: {e}"));
                        }
                    }
                }
            }
        }
    }
}
