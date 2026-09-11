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
    let Some(counter) = dt.counters.counters.get(dt.counters.counter_index) else {
        frame.render_widget(Paragraph::new("No counter at this index.\nIt's likely that it was removed.\nHit Esc to pick another one.").bold().red(), area);
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
        counter_layout[1].centered_vertically(Constraint::Max(8)),
    );
    render_gauge(&counter, frame, counter_layout[2]);
}

fn render_value(counter: &Counter, frame: &mut Frame, area: Rect) {
    let (pixel_size, constraint) = pixel_size(counter, &area);
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

fn pixel_size(counter: &Counter, area: &Rect) -> (PixelSize, Constraint) {
    match (counter.value, area.width) {
        (..100000, 50..) => (PixelSize::Full, Constraint::Length(8)),
        (..100000, ..50) => (PixelSize::Quadrant, Constraint::Length(4)),
        (100000.., 50..) => (PixelSize::HalfWidth, Constraint::Length(8)),
        (100000.., ..50) => (PixelSize::Octant, Constraint::Length(2)),
    }
}

fn render_gauge(counter: &Counter, frame: &mut Frame, area: Rect) {
    let ratio = ((counter.value as f64 - counter.min_val as f64)
        / (counter.max_val as f64 - counter.min_val as f64))
        .clamp(0.0, 1.0);
    let counter_gauge = Gauge::default().ratio(ratio);
    if area.width >= 50 {
        let counter_gauge_layout = Layout::horizontal([Constraint::Fill(1); 3]).split(area);
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
        KeyCode::Char('q') => dt.state = DumbTallyAppState::Quit,
        KeyCode::Char('=') => change_current_counter_val(dt, ChangeCounterValue::Increase),
        KeyCode::Char('-') => change_current_counter_val(dt, ChangeCounterValue::Decrease),
        KeyCode::Char('g') => change_current_counter_val(dt, ChangeCounterValue::Min),
        KeyCode::Char('G') => change_current_counter_val(dt, ChangeCounterValue::Max),
        KeyCode::Char('r') => change_current_counter_val(dt, ChangeCounterValue::Reset),
        KeyCode::Char('n') => dt.state = DumbTallyAppState::NewCounter,
        KeyCode::Esc => dt.state = DumbTallyAppState::CounterPick,
        _ => {}
    }
}

fn change_current_counter_val(dt: &mut DumbTallyApp, change: ChangeCounterValue) {
    let Some(counter) = dt.counters.counters.get_mut(dt.counters.counter_index) else {
        return;
    };
    counter.value = match change {
        ChangeCounterValue::Increase => {
            (counter.value + counter.pos_add).clamp(counter.min_val, counter.max_val)
        }
        ChangeCounterValue::Decrease => {
            (counter.value - counter.pos_add).clamp(counter.min_val, counter.max_val)
        }
        ChangeCounterValue::Min => counter.min_val,
        ChangeCounterValue::Max => counter.max_val,
        ChangeCounterValue::Reset => {
            if counter.min_val > 0 {
                counter.min_val
            } else {
                0
            }
        }
    }
}

enum ChangeCounterValue {
    Increase,
    Decrease,
    Min,
    Max,
    Reset,
}
