use std::sync::{Arc, Mutex};

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
    Frame,
};

use crate::{
    spectrum::{IqCapture, SpectrumBins, FFT_SIZE},
    state::RadioState,
    ui::component::Component,
};

pub struct Spectrum {
    bins: Option<SpectrumBins>,
    sample_rate: u32,
    is_stereo: Option<Arc<Mutex<bool>>>,
}

impl Spectrum {
    pub fn new(iq: Option<&IqCapture>) -> Self {
        match iq {
            Some(capture) => Self {
                bins: Some(capture.bins.clone()),
                sample_rate: capture.sample_rate,
                is_stereo: Some(capture.is_stereo.clone()),
            },
            None => Self {
                bins: None,
                sample_rate: 0,
                is_stereo: None,
            },
        }
    }
}

impl Component for Spectrum {
    fn constraint(&self) -> Constraint {
        if self.bins.is_some() {
            Constraint::Length(10)
        } else {
            Constraint::Length(0)
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, state: &RadioState) {
        let Some(bins) = &self.bins else { return };
        let Ok(bins) = bins.lock() else { return };

        let inner_w = area.width.saturating_sub(2) as usize;
        if inner_w == 0 {
            return;
        }

        let stereo = self
            .is_stereo
            .as_ref()
            .and_then(|s| s.lock().map(|v| *v).ok())
            .unwrap_or(false);

        let (display_bins, bw_label) = if stereo {
            let bw_khz = self.sample_rate / 2 / 1000;
            (bins.as_slice(), format!("±{bw_khz} kHz"))
        } else {
            let bw_khz = self.sample_rate / 2 / 1000;
            (&bins[..FFT_SIZE / 2], format!("0\u{2013}{bw_khz} kHz"))
        };

        let dc_label = if state.dc_suppress { " DC∅" } else { "" };
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

        if stereo && state.dc_suppress {
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

    for (i, slot) in data
        .iter_mut()
        .enumerate()
        .take(null_end + 1)
        .skip(null_start)
    {
        let t = (i.saturating_sub(ref_l)) as f64 / span;
        let baseline = v_l + t * (v_r - v_l);
        let h = (i as u32).wrapping_mul(2654435761u32);
        let noise = (h as f64 / u32::MAX as f64 - 0.5) * noise_amp;
        *slot = (baseline + noise).max(0.0) as u64;
    }
}
