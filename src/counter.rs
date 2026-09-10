use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Gauge, Paragraph},
};
use tui_big_text::{BigText, PixelSize};

use crate::dt::{Counter, DumbTallyApp, DumbTallyAppState};

pub fn render_counter(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let Some(counter) = dt.counters.get(dt.counter_index) else {
        frame.render_widget(Paragraph::new("No counter"), area);
        return;
    };
    let counter_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(area);
    let counter_name = Paragraph::new(format!("Current counter: {}", counter.name)).bold();
    frame.render_widget(counter_name, counter_layout[0]);
    render_value(
        &counter,
        frame,
        counter_layout[1].centered_vertically(Constraint::Length(4)),
    );
    render_gauge(&counter, frame, counter_layout[2]);
}

fn render_value(counter: &Counter, frame: &mut Frame, area: Rect) {
    let (pixel_size, constraint) = if area.width > 50 {
        (PixelSize::HalfHeight, Constraint::Length(4))
    } else {
        (PixelSize::Octant, Constraint::Length(2))
    };
    let mut counter_value = BigText::builder()
        .centered()
        .pixel_size(pixel_size)
        .lines(vec![format!("{}", counter.value).into()])
        .build();
    if counter.value >= counter.max_val {
        counter_value.block = Some(Block::default().on_green().black());
    } else if counter.value <= counter.min_val {
        counter_value.block = Some(Block::default().on_red().black());
    }
    frame.render_widget(counter_value, area.centered_vertically(constraint));
}

fn render_gauge(counter: &Counter, frame: &mut Frame, area: Rect) {
    let ratio = ((counter.value as f64 - counter.min_val as f64)
        / (counter.max_val as f64 - counter.min_val as f64))
        .clamp(0.0, 1.0);
    let counter_gauge = Gauge::default().ratio(ratio);
    if area.width >= 50 {
        let counter_gauge_layout = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(area);
        let counter_gauge_minval = Paragraph::new(format!("{}", counter.min_val)).centered();
        let counter_gauge_maxval = Paragraph::new(format!("{}", counter.max_val)).centered();
        frame.render_widget(counter_gauge, counter_gauge_layout[1]);
        frame.render_widget(counter_gauge_minval, counter_gauge_layout[0]);
        frame.render_widget(counter_gauge_maxval, counter_gauge_layout[2]);
    } else {
        frame.render_widget(counter_gauge, area);
    }
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('=') => increase_current_counter_val(dt),
        KeyCode::Char('-') => decrease_current_counter_val(dt),
        KeyCode::Char('n') => dt.state = DumbTallyAppState::NewCounter,
        KeyCode::Esc => dt.state = DumbTallyAppState::CounterPick,
        _ => {}
    }
}

fn increase_current_counter_val(dt: &mut DumbTallyApp) {
    let Some(counter) = dt.counters.get_mut(dt.counter_index) else {
        return;
    };
    counter.value = (counter.value + counter.pos_add).clamp(counter.min_val, counter.max_val);
}

fn decrease_current_counter_val(dt: &mut DumbTallyApp) {
    let Some(counter) = dt.counters.get_mut(dt.counter_index) else {
        return;
    };
    counter.value = (counter.value - counter.neg_add).clamp(counter.min_val, counter.max_val);
}
