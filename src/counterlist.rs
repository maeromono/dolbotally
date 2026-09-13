use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::List,
};

use crate::{counteredit::CounterEdit, dt::{DumbTallyApp, DumbTallyAppState}};

pub fn render_counter_list(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let mut counter_list: Vec<String> = vec![];
    for counter in &dt.counters.counters {
        counter_list.push(format!("[{}] {}", counter.value, counter.name));
    }
    let list = List::new(counter_list).highlight_style(Modifier::REVERSED);
    frame.render_stateful_widget(
        list,
        area.centered(Constraint::Max(75), Constraint::Percentage(30)),
        &mut dt.counter_list,
    );
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('q') => dt.state = DumbTallyAppState::Quit,
        KeyCode::Esc => dt.state = DumbTallyAppState::Main,
        KeyCode::Char('j') | KeyCode::Down => dt.counter_list.select_next(),
        KeyCode::Char('k') | KeyCode::Up => dt.counter_list.select_previous(),
        KeyCode::Enter | KeyCode::Char(' ') => {
            dt.counters.counter_index = dt.counter_list.selected().unwrap_or_default();
            dt.state = DumbTallyAppState::Main;
        }
        KeyCode::Char('e') => {
            dt.counters.counter_index = dt.counter_list.selected().unwrap_or_default();
            let Some(counter_edit) = CounterEdit::edit(dt, dt.counters.counter_index) else {
                return;
            };
            dt.new_counter = counter_edit;
            dt.state = DumbTallyAppState::NewCounter;
        }
        _ => {}
    }
}
