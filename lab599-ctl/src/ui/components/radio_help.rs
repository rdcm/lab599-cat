use ratatui::{
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{state::RadioState, ui::component::Component};

use super::helpers::entry;

pub struct RadioHelp;

impl Component for RadioHelp {
    fn constraint(&self) -> Constraint {
        Constraint::Percentage(50)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, _state: &RadioState) {
        let lines = vec![
            entry("← / →    ", "Tune by current step"),
            entry("↑ / ↓    ", "Change tuning step (10 Hz … 10 kHz)"),
            entry("+ / -    ", "Tune ±1 MHz"),
            entry("[ / ]    ", "Band down / up"),
            Line::from(""),
            entry("m        ", "Cycle mode  LSB→USB→CW→CW-R→AM→FM→DIG"),
            entry("f        ", "Cycle filter (FIL-1 … FIL-4)"),
            entry("t        ", "Toggle PTT  (TX / RX)"),
            entry("p        ", "Toggle preamp"),
            entry("a        ", "Toggle attenuator"),
            entry("s        ", "Toggle split VFO"),
            entry("c        ", "Toggle CMR (carrier cancel)"),
            entry("v        ", "Toggle VOX"),
            entry("n        ", "Toggle noise reduction (NR)"),
            entry("b        ", "Toggle noise blanker (NB)"),
            entry("x        ", "Toggle notch filter"),
            entry("o        ", "Toggle monitor (sidetone)"),
            entry("d        ", "Toggle DIF (duplex IF shift)"),
        ];

        frame.render_widget(
            Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Radio ")),
            area,
        );
    }
}
