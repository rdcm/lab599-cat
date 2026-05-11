use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use crate::state::RadioState;

pub struct AppLayout;

impl AppLayout {
    pub fn render(
        frame: &mut Frame,
        state: &RadioState,
        page_names: &[&str],
        current: usize,
    ) -> Rect {
        let model_title = Line::from(vec![
            Span::raw(" Lab599 "),
            Span::styled(
                state.model.to_string(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]);

        let mut tab_spans: Vec<Span> = vec![Span::raw(" ")];
        for (i, name) in page_names.iter().enumerate() {
            if i > 0 {
                tab_spans.push(Span::raw(" │ "));
            }
            if i == current {
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
        let tabs_title = Line::from(tab_spans).right_aligned();

        let outer = Block::default()
            .borders(Borders::ALL)
            .title(model_title)
            .title(tabs_title);
        let inner = outer.inner(frame.area());
        frame.render_widget(outer, frame.area());

        inner
    }
}
