use std::sync::{Arc, Mutex};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline, Widget},
};

use crate::services::spectrum::{Bins, FFT_SIZE};
use crate::ui::ui_utils::suppress_lo_spike;

pub struct SpectrumWidget {
    bins: Bins,
    sample_rate: u32,
    is_stereo: Arc<Mutex<bool>>,
    dc_suppress: bool,
}

impl SpectrumWidget {
    pub fn new(
        bins: Bins,
        sample_rate: u32,
        is_stereo: Arc<Mutex<bool>>,
        dc_suppress: bool,
    ) -> Self {
        Self {
            bins,
            sample_rate,
            is_stereo,
            dc_suppress,
        }
    }
}

impl Widget for SpectrumWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Ok(bins) = self.bins.lock() else { return };

        let inner_w = area.width.saturating_sub(2) as usize;
        if inner_w == 0 {
            return;
        }

        let stereo = self.is_stereo.lock().map(|v| *v).unwrap_or(false);

        let (display_bins, bw_label) = if stereo {
            let bw_khz = self.sample_rate / 2 / 1000;
            (bins.as_slice(), format!("±{bw_khz} kHz"))
        } else {
            let bw_khz = self.sample_rate / 2 / 1000;
            (&bins[..FFT_SIZE / 2], format!("0\u{2013}{bw_khz} kHz"))
        };

        let dc_label = if self.dc_suppress { " DC∅" } else { "" };
        let title = if stereo {
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
                (max_db + 120.0).clamp(0.0, 100.0) as u64
            })
            .collect();

        if stereo && self.dc_suppress {
            suppress_lo_spike(&mut data);
        }

        Sparkline::default()
            .block(Block::default().borders(Borders::ALL).title(title))
            .data(&data)
            .max(100)
            .style(Style::default().fg(Color::Green))
            .render(area, buf);
    }
}
