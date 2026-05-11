use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event};

use crate::{
    audio::Audio,
    config::Config,
    events::{action::Action, bus::EventBus},
    pages::router::Router,
    radio::{open_port, Radio},
    spectrum::IqCapture,
    state::{Model, RadioState},
};

pub struct App {
    radio: Radio,
    state: RadioState,
    router: Router,
    bus: EventBus,
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
            bus: EventBus::new(),
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
        result
    }

    fn event_loop(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        let mut last_poll = Instant::now();

        loop {
            terminal.draw(|f| self.router.render(f, &self.state))?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    self.router.handle_event(&key, &mut self.bus);
                }
            }

            for action in self.bus.drain() {
                if matches!(action, Action::Quit) {
                    return Ok(());
                }
                self.radio.handle(action, &mut self.state);
            }

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
