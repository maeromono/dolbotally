use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::List,
};

use crate::dt::{DumbTallyApp, DumbTallyAppState};

pub fn render_counter_list(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let mut counter_list: Vec<String> = vec![];
    for counter in &dt.counters {
        counter_list.push(format!("[{}] {}", counter.value, counter.name));
    }
    let list = List::new(counter_list).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list,
        area.centered(Constraint::Max(75), Constraint::Percentage(30)),
        &mut dt.counter_list,
    );
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyCode) {
    match key {
        KeyCode::Esc => dt.state = DumbTallyAppState::Main,
        KeyCode::Char('j') | KeyCode::Down => dt.counter_list.select_next(),
        KeyCode::Char('k') | KeyCode::Up => dt.counter_list.select_previous(),
        KeyCode::Enter | KeyCode::Char(' ') => {
            dt.counter_index = dt.counter_list.selected().unwrap_or_default();
            dt.state = DumbTallyAppState::Main;
        }
        _ => {}
    }
}
