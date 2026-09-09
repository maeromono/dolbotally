use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Gauge, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

use crate::dt::DumbTallyApp;

pub fn render_counter(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let Some(counter) = dt.counters.get(dt.counter_index) else {
        frame.render_widget(Paragraph::new("No counter"), area);
        return;
    };
    let counter_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .split(area);
    let counter_name = Paragraph::new(format!("Current counter: {}", counter.name)).bold();
    let counter_value = BigText::builder()
        .centered()
        .pixel_size(PixelSize::Octant)
        .lines(vec![format!("{}", counter.value).into()])
        .build();
    let ratio = ((counter.value as f64 - counter.min_val as f64)
        / (counter.max_val as f64 - counter.min_val as f64))
        .clamp(0.0, 1.0);
    let counter_gauge = Gauge::default().ratio(ratio);
    frame.render_widget(counter_name, counter_layout[0]);
    frame.render_widget(counter_value, counter_layout[1]);
    frame.render_widget(counter_gauge, counter_layout[2]);
}
