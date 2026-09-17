use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Wrap},
};
use ratatui_textarea::TextArea;

use crate::dt::{DumbTallyApp, DumbTallyAppState};

pub struct CounterDelete {
    pub counter_delete_textarea: TextArea<'static>,
    pub deletion_input_valid: bool,
}

impl Default for CounterDelete {
    fn default() -> Self {
        Self {
            counter_delete_textarea: TextArea::default(),
            deletion_input_valid: false,
        }
    }
}

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
    let delete_layout = Layout::vertical([Constraint::Max(3), Constraint::Length(1)])
        .split(area.centered(Constraint::Max(75), Constraint::Percentage(30)));
    let delete_confirmation_message = Paragraph::new(format!(
        "Delete counter \"{}\"?\nType \"Yes, please, I want it gone!\" to confirm.",
        counter.name
    ))
    .wrap(Wrap { trim: false })
    .red();
    frame.render_widget(delete_confirmation_message, delete_layout[0]);
    frame.render_widget(&dt.counter_delete.counter_delete_textarea, delete_layout[1]);
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            dt.state = DumbTallyAppState::CounterPick;
            reset_textareas(dt);
        }
        KeyCode::Enter => {
            confirm_delete(dt);
            reset_textareas(dt);
        }
        _ => {
            let textarea = &mut dt.counter_delete.counter_delete_textarea;
            if textarea.input(*key) {
                dt.counter_delete.deletion_input_valid = validate_input(textarea);
            }
        }
    }
}

fn validate_input(textarea: &mut TextArea) -> bool {
    let text = &textarea.lines()[0];
    if text != "Yes, please, I want it gone!" {
        textarea.set_style(Style::default().fg(Color::Black).bg(Color::Red));
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

fn confirm_delete(dt: &mut DumbTallyApp) {
    if dt.counter_delete.deletion_input_valid {
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
}

fn reset_textareas(dt: &mut DumbTallyApp) {
    dt.counter_delete = CounterDelete::default();
}
