use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Stylize},
    widgets::{List, Paragraph},
};

use crate::{
    counteredit::CounterEdit,
    dt::{DumbTallyApp, DumbTallyAppState},
};

pub fn render_counter_list(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let mut counter_list: Vec<String> = vec![];
    if dt.counters.counters.len() == 0 {
        frame.render_widget(
            Paragraph::new("No counters.\nHit Esc, then n\nto make a new counter.")
                .red()
                .bold(),
            area,
        );
        return;
    }
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
            dt.counter_edit = counter_edit;
            dt.state = DumbTallyAppState::CounterEdit;
        }
        KeyCode::Char('d') => {
            let mut counter_index = dt.counter_list.selected().unwrap_or_default();
            dt.counters.counters.remove(counter_index);
            if dt.counters.counters.len() == 0 {
                dt.state = DumbTallyAppState::CounterEdit;
            } else if counter_index >= dt.counters.counters.len() {
                counter_index -= 1;
                dt.counters.counter_index = counter_index;
            }
            dt.save();
        }
        _ => {}
    }
}
