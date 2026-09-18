use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Stylize,
    widgets::{Paragraph, Wrap},
};

use crate::dt::{DumbTallyApp, DumbTallyAppState};

pub fn render_counterdelete_screen(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let Some(counter) = dt.counters.counters.get(dt.counters.counter_index) else {
        frame.render_widget(
            Paragraph::new(
                "No counter at this index.\nIt's likely that it was removed.\nHit Esc to cancel.",
            )
            .bold()
            .red(),
            area,
        );
        return;
    };
    let delete_confirmation_message = Paragraph::new(format!(
        "Permanently delete counter \"{}\"?\nPress y to confirm or n to cancel.",
        counter.name
    ))
    .wrap(Wrap { trim: false })
    .yellow().bold();
    frame.render_widget(
        delete_confirmation_message,
        area.centered(Constraint::Max(75), Constraint::Percentage(30)),
    );
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    match key.code {
        KeyCode::Char('n') => {
            dt.state = DumbTallyAppState::CounterPick;
        }
        KeyCode::Char('y') => {
            confirm_delete(dt);
        }
        _ => {}
    }
}

fn confirm_delete(dt: &mut DumbTallyApp) {
    let mut counter_index = dt.counters.counter_index;
    dt.counters.counters.remove(counter_index);
    if dt.counters.counters.len() == 0 {
        dt.state = DumbTallyAppState::CounterEdit;
    } else if counter_index >= dt.counters.counters.len() {
        counter_index -= 1;
        dt.counters.counter_index = counter_index;
        dt.state = DumbTallyAppState::CounterPick;
    } else {
        dt.state = DumbTallyAppState::CounterPick;
    }
    dt.save();
}
