use crate::hardware::radio::Radio;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub const KEY: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);

pub fn entry<'a>(key: &'a str, desc: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::raw("  "),
        Span::styled(key, KEY),
        Span::raw(desc),
    ])
}

pub fn suppress_lo_spike(data: &mut [u64]) {
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

pub fn apply_key(key: KeyEvent, radio: &mut Radio) {
    match (key.code, key.modifiers) {
        (KeyCode::Right, _) => {
            let d = radio.state().step.hz() as i64;
            radio.tune(d);
        }
        (KeyCode::Left, _) => {
            let d = radio.state().step.hz() as i64;
            radio.tune(-d);
        }
        (KeyCode::Up, _) => radio.step_next(),
        (KeyCode::Down, _) => radio.step_prev(),
        (KeyCode::PageUp, _) | (KeyCode::Char('+'), _) => radio.tune(1_000_000),
        (KeyCode::PageDown, _) | (KeyCode::Char('-'), _) => radio.tune(-1_000_000),
        (KeyCode::Char('m'), _) => radio.toggle_mode(),
        (KeyCode::Char('f'), _) => radio.toggle_filter(),
        (KeyCode::Char('t'), _) => radio.toggle_ptt(),
        (KeyCode::Char('p'), _) => radio.toggle_preamp(),
        (KeyCode::Char('a'), _) => radio.toggle_attenuator(),
        (KeyCode::Char('s'), _) => radio.toggle_split(),
        (KeyCode::Char('c'), _) => radio.toggle_cmr(),
        (KeyCode::Char('v'), _) => radio.toggle_vox(),
        (KeyCode::Char('n'), _) => radio.toggle_nr(),
        (KeyCode::Char('b'), _) => radio.toggle_nb(),
        (KeyCode::Char('x'), _) => radio.toggle_notch(),
        (KeyCode::Char('o'), _) => radio.toggle_mon(),
        (KeyCode::Char('d'), _) => radio.toggle_dif(),
        (KeyCode::Char('z'), _) => radio.toggle_dc_suppress(),
        (KeyCode::Char('['), _) => radio.band_down(),
        (KeyCode::Char(']'), _) => radio.band_up(),
        _ => {}
    }
}
