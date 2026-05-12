use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::KeyCode;

use crate::hardware::audio::Audio;
use crate::hardware::radio::{open_port, Radio};
use crate::input::keyboard::{Keyboard, Quit};
use crate::ui::layout::AppLayout;
use crate::ui::router::Router;
use crate::{
    config::Config,
    hardware::spectrum::IqCapture,
    hardware::state::{Model, RadioState},
};

pub struct App {
    radio: Radio,
    state: RadioState,
    router: Router,
    _audio: Option<Audio>,
    _iq: Option<IqCapture>,
    poll_interval: Duration,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        let mut radio = Radio::new(lab599_cat::CatDriver::new(open_port(
            &config.port,
            config.baud,
        )?));

        let audio = config
            .audio_device
            .as_deref()
            .and_then(|name| Audio::new(name, &config.rx_socket));

        let mut state = RadioState {
            audio_active: audio.is_some(),
            model: radio.get_id().map(Model::from).unwrap_or_default(),
            ..Default::default()
        };

        let iq = config.iq_device.as_deref().and_then(|name| {
            IqCapture::new(name, config.iq_rate)
                .map_err(|e| state.log_error(format!("IQ: {e}")))
                .ok()
        });

        radio.tick(&mut state);

        Ok(Self {
            router: Router::new(iq.as_ref()),
            poll_interval: Duration::from_millis(config.poll_ms),
            _audio: audio,
            _iq: iq,
            radio,
            state,
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

        loop {
            let page_key = match Keyboard::read_key(50)? {
                Some(k) if k.code == KeyCode::Tab => {
                    self.router.advance_page();
                    None
                }
                key => key,
            };

            terminal.draw(|f| {
                let names = self.router.page_names();
                let inner = AppLayout::render(f, &self.state, &names, self.router.current());
                if let Some(page) = self.router.current_page_mut() {
                    let action = page.render(f, inner, &self.state, page_key);
                    if let Some(a) = action {
                        self.radio.handle(a, &mut self.state);
                    }
                }
            })?;

            if last_poll.elapsed() >= self.poll_interval {
                self.radio.tick(&mut self.state);
                last_poll = Instant::now();
                if let Some(iq) = &self._iq {
                    if let Ok(mut errs) = iq.errors.lock() {
                        for e in errs.drain(..) {
                            self.state.log_error(format!("IQ: {e}"));
                        }
                    }
                }
            }
        }
    }
}
