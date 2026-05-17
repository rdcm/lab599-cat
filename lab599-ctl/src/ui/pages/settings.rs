use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};

use super::page::Page;
use crate::app_state::AppState;
use crate::ui::components::spectrum::processor;
use crate::ui::widgets::settings::SettingsWidget;

const BAUD_RATES: [u32; 6] = [4_800, 9_600, 19_200, 38_400, 57_600, 115_200];
const IQ_RATES: [u32; 5] = [44_100, 48_000, 96_000, 192_000, 384_000];
const POLL_INTERVALS: [u64; 6] = [50, 100, 200, 500, 1_000, 2_000];

// Cursor row indices
const ROW_AUDIO_TOGGLE: usize = 0;
const ROW_AUDIO_DEV: usize = 1;
const ROW_IQ_TOGGLE: usize = 2;
const ROW_IQ_DEV: usize = 3;
const ROW_IQ_RATE: usize = 4;
const ROW_REMOTE_TOGGLE: usize = 5;
const ROW_CAT_POLL: usize = 6;
const ROW_CAT_BAUD: usize = 7;
const ROW_CAT_RECONNECT: usize = 8;
const ROW_MAX: usize = 8;

pub struct SettingsPage {
    devices: Vec<String>,
    audio_dev_idx: usize,
    iq_dev_idx: usize,
    iq_rate_idx: usize,
    baud_idx: usize,
    poll_idx: usize,
    cursor: usize,
}

impl SettingsPage {
    pub fn new(active_baud: u32, poll_ms: u64) -> Self {
        let baud_idx = BAUD_RATES
            .iter()
            .position(|&b| b == active_baud)
            .unwrap_or(1);
        let poll_idx = POLL_INTERVALS
            .iter()
            .position(|&p| p == poll_ms)
            .unwrap_or(2);
        let devices = processor::list_iq_devices();
        let iq_rate_idx = IQ_RATES.iter().position(|&r| r == 48_000).unwrap_or(1);

        Self {
            devices,
            audio_dev_idx: 0,
            iq_dev_idx: 0,
            iq_rate_idx,
            baud_idx,
            poll_idx,
            cursor: 0,
        }
    }

    fn refresh_devices(&mut self) {
        self.devices = processor::list_iq_devices();
        if self.audio_dev_idx >= self.devices.len() {
            self.audio_dev_idx = 0;
        }
        if self.iq_dev_idx >= self.devices.len() {
            self.iq_dev_idx = 0;
        }
    }

    fn audio_dev(&self) -> &str {
        self.devices
            .get(self.audio_dev_idx)
            .map(String::as_str)
            .unwrap_or("—")
    }

    fn iq_dev(&self) -> &str {
        self.devices
            .get(self.iq_dev_idx)
            .map(String::as_str)
            .unwrap_or("—")
    }

    fn cycle_left(idx: &mut usize, len: usize) {
        if len > 0 {
            *idx = if *idx == 0 { len - 1 } else { *idx - 1 };
        }
    }

    fn cycle_right(idx: &mut usize, len: usize) {
        if len > 0 {
            *idx = (*idx + 1) % len;
        }
    }
}

impl Page for SettingsPage {
    fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        app_state: &mut AppState,
        key: Option<KeyEvent>,
    ) {
        if let Some(k) = key {
            match k.code {
                KeyCode::Up => {
                    self.cursor = self.cursor.saturating_sub(1);
                }
                KeyCode::Down if self.cursor < ROW_MAX => {
                    self.cursor += 1;
                }

                KeyCode::Enter => match self.cursor {
                    ROW_AUDIO_TOGGLE => {
                        if app_state.audio.is_active() {
                            app_state.audio.stop_audio();
                            app_state.radio.state_mut().audio_active = false;
                        } else if let Some(name) = self.devices.get(self.audio_dev_idx).cloned() {
                            match app_state.audio.start_loopback(&name) {
                                Ok(()) => app_state.radio.state_mut().audio_active = true,
                                Err(e) => app_state.radio.log_error(format!("Audio: {e}")),
                            }
                        }
                    }
                    ROW_IQ_TOGGLE => {
                        if app_state.spectrum.is_active() {
                            app_state.spectrum.stop();
                        } else if let Some(name) = self.devices.get(self.iq_dev_idx).cloned() {
                            let rate = IQ_RATES[self.iq_rate_idx];
                            app_state.iq_rate = rate;
                            let errors = app_state.audio.errors().clone();
                            if let Err(e) = app_state.spectrum.start(&name, rate, errors) {
                                app_state.radio.log_error(format!("Spectrum: {e}"));
                            }
                        }
                    }
                    ROW_REMOTE_TOGGLE => {
                        if app_state.audio.is_remote_active() {
                            app_state.audio.stop_remote();
                        } else {
                            match app_state.audio.start_remote() {
                                Ok(()) => {}
                                Err(e) => app_state.radio.log_error(format!("Remote: {e}")),
                            }
                        }
                    }
                    ROW_CAT_RECONNECT => {
                        let baud = BAUD_RATES[self.baud_idx];
                        match app_state.radio.reconnect_blocking(baud) {
                            Ok(()) => {}
                            Err(e) => app_state.radio.log_error(format!("CAT reconnect: {e}")),
                        }
                    }
                    _ => {}
                },

                KeyCode::Left => match self.cursor {
                    ROW_AUDIO_DEV if !app_state.audio.is_active() => {
                        Self::cycle_left(&mut self.audio_dev_idx, self.devices.len());
                    }
                    ROW_IQ_DEV if !app_state.spectrum.is_active() => {
                        Self::cycle_left(&mut self.iq_dev_idx, self.devices.len());
                    }
                    ROW_IQ_RATE => {
                        Self::cycle_left(&mut self.iq_rate_idx, IQ_RATES.len());
                    }
                    ROW_CAT_BAUD => {
                        Self::cycle_left(&mut self.baud_idx, BAUD_RATES.len());
                    }
                    ROW_CAT_POLL => {
                        Self::cycle_left(&mut self.poll_idx, POLL_INTERVALS.len());
                        app_state.poll_interval =
                            Duration::from_millis(POLL_INTERVALS[self.poll_idx]);
                    }
                    _ => {}
                },

                KeyCode::Right => match self.cursor {
                    ROW_AUDIO_DEV if !app_state.audio.is_active() => {
                        Self::cycle_right(&mut self.audio_dev_idx, self.devices.len());
                    }
                    ROW_IQ_DEV if !app_state.spectrum.is_active() => {
                        Self::cycle_right(&mut self.iq_dev_idx, self.devices.len());
                    }
                    ROW_IQ_RATE => {
                        Self::cycle_right(&mut self.iq_rate_idx, IQ_RATES.len());
                    }
                    ROW_CAT_BAUD => {
                        Self::cycle_right(&mut self.baud_idx, BAUD_RATES.len());
                    }
                    ROW_CAT_POLL => {
                        Self::cycle_right(&mut self.poll_idx, POLL_INTERVALS.len());
                        app_state.poll_interval =
                            Duration::from_millis(POLL_INTERVALS[self.poll_idx]);
                    }
                    _ => {}
                },

                KeyCode::Char('R') | KeyCode::Char('r')
                    if self.cursor == ROW_AUDIO_DEV || self.cursor == ROW_IQ_DEV =>
                {
                    self.refresh_devices();
                }

                _ => {}
            }
        }

        let audio_device = if app_state.audio.is_active() {
            app_state.audio.active_device().unwrap_or("—").to_string()
        } else {
            self.audio_dev().to_string()
        };
        let iq_device = if app_state.spectrum.is_active() {
            app_state
                .spectrum
                .device_name
                .as_deref()
                .unwrap_or("—")
                .to_string()
        } else {
            self.iq_dev().to_string()
        };

        frame.render_widget(
            SettingsWidget {
                cursor: self.cursor,
                audio_active: app_state.audio.is_active(),
                audio_device,
                audio_device_editable: !app_state.audio.is_active(),
                iq_active: app_state.spectrum.is_active(),
                iq_device,
                iq_device_editable: !app_state.spectrum.is_active(),
                iq_rate: IQ_RATES[self.iq_rate_idx],
                remote_active: app_state.audio.is_remote_active(),
                rx_socket: app_state.audio.rx_socket_path().to_string(),
                cat_serial_port: app_state.radio.active_port().to_string(),
                cat_baud: BAUD_RATES[self.baud_idx],
                cat_poll_ms: POLL_INTERVALS[self.poll_idx],
            },
            area,
        );
    }
}
