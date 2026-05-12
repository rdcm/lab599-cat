use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
    Frame,
};

use crate::hardware::state::RadioState;

struct AppBarWidget {
    model: String,
    tabs: Vec<&'static str>,
    current: usize,
}

impl AppBarWidget {
    fn new(state: &RadioState, tabs: &[&'static str], current: usize) -> Self {
        Self {
            model: state.model.to_string(),
            tabs: tabs.to_vec(),
            current,
        }
    }
}

impl Widget for AppBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let model_title = Line::from(vec![
            Span::raw(" Lab599 "),
            Span::styled(
                self.model,
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]);

        let mut tab_spans: Vec<Span> = vec![Span::raw(" ")];
        for (i, name) in self.tabs.iter().enumerate() {
            if i > 0 {
                tab_spans.push(Span::raw(" │ "));
            }
            if i == self.current {
                tab_spans.push(Span::styled(
                    *name,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                tab_spans.push(Span::raw(*name));
            }
        }
        tab_spans.push(Span::raw(" "));

        Block::default()
            .borders(Borders::ALL)
            .title(model_title)
            .title(Line::from(tab_spans).right_aligned())
            .render(area, buf);
    }
}

pub struct AppBar;

impl AppBar {
    pub fn render(
        frame: &mut Frame,
        state: &RadioState,
        tabs: &[&'static str],
        current: usize,
    ) -> Rect {
        let inner = Block::default().borders(Borders::ALL).inner(frame.area());
        frame.render_widget(AppBarWidget::new(state, tabs, current), frame.area());
        inner
    }
}
