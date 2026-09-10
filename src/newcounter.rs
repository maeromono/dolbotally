use crate::dt::{Counter, DumbTallyApp, DumbTallyAppState};
use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
};
use ratatui_textarea::TextArea;

#[derive(Default)]
pub struct NewCounter {
    current_focus: CurrentFocus,
    counter_name_textarea: TextArea<'static>,
    counter_posadd_textarea: TextArea<'static>,
    counter_negadd_textarea: TextArea<'static>,
    counter_minval_textarea: TextArea<'static>,
    counter_maxval_textarea: TextArea<'static>,
    counter_posadd_valid: bool,
    counter_negadd_valid: bool,
    counter_minval_valid: bool,
    counter_maxval_valid: bool,
}

#[derive(Default, Sequence)]
pub enum CurrentFocus {
    #[default]
    Name,
    PosAdd,
    NegAdd,
    MinVal,
    MaxVal,
}

pub fn render_countermake_screen(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    let newcounter_layout = Layout::vertical([Constraint::Length(2); 5])
        .split(area.centered_vertically(Constraint::Max(10)));
    frame.render_widget(&dt.new_counter.counter_name_textarea, newcounter_layout[0]);
    frame.render_widget(
        &dt.new_counter.counter_posadd_textarea,
        newcounter_layout[1],
    );
    frame.render_widget(
        &dt.new_counter.counter_negadd_textarea,
        newcounter_layout[2],
    );
    frame.render_widget(
        &dt.new_counter.counter_minval_textarea,
        newcounter_layout[3],
    );
    frame.render_widget(
        &dt.new_counter.counter_maxval_textarea,
        newcounter_layout[4],
    );
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    // if dt.new_counter.counter_name_textarea.input_without_shortcuts(*key) {
    //
    // }
    match key.code {
        KeyCode::Esc => {
            dt.state = DumbTallyAppState::Main;
            reset_textareas(dt);
        }
        KeyCode::Tab => dt.new_counter.current_focus = next_cycle(&dt.new_counter.current_focus),
        KeyCode::Enter => create_new_counter(dt),
        _ => match dt.new_counter.current_focus {
            CurrentFocus::Name => if dt.new_counter.counter_name_textarea.input(*key) {},
            CurrentFocus::PosAdd => {
                let textarea = &mut dt.new_counter.counter_posadd_textarea;
                if textarea.input(*key) {
                    dt.new_counter.counter_posadd_valid = validate_ints(textarea);
                }
            }
            CurrentFocus::NegAdd => {
                let textarea = &mut dt.new_counter.counter_negadd_textarea;
                if textarea.input(*key) {
                    dt.new_counter.counter_negadd_valid = validate_ints(textarea);
                }
            }
            CurrentFocus::MinVal => {
                let textarea = &mut dt.new_counter.counter_minval_textarea;
                if textarea.input(*key) {
                    dt.new_counter.counter_minval_valid = validate_ints(textarea);
                }
            }
            CurrentFocus::MaxVal => {
                let textarea = &mut dt.new_counter.counter_maxval_textarea;
                if textarea.input(*key) {
                    dt.new_counter.counter_maxval_valid = validate_ints(textarea);
                }
            }
        },
    }
}

fn validate_ints(textarea: &mut TextArea) -> bool {
    if let Err(err) = textarea.lines()[0].parse::<i64>() {
        textarea.set_style(Style::default().fg(Color::Black).bg(Color::Red));
        false
    } else {
        textarea.set_style(Style::default());
        true
    }
}

fn create_new_counter(dt: &mut DumbTallyApp) {
    if dt.new_counter.counter_name_textarea.lines()[0].len() <= 40
        && dt.new_counter.counter_posadd_valid
        && dt.new_counter.counter_negadd_valid
        && dt.new_counter.counter_minval_valid
        && dt.new_counter.counter_maxval_valid
    {
        let min_val = dt.new_counter.counter_minval_textarea.lines()[0]
            .parse()
            .unwrap();
        dt.counters.push(Counter {
            name: dt.new_counter.counter_name_textarea.lines()[0].clone(),
            value: if min_val > 0 { min_val } else { 0 },
            pos_add: dt.new_counter.counter_posadd_textarea.lines()[0]
                .parse()
                .unwrap(),
            neg_add: dt.new_counter.counter_negadd_textarea.lines()[0]
                .parse()
                .unwrap(),
            min_val,
            max_val: dt.new_counter.counter_maxval_textarea.lines()[0]
                .parse()
                .unwrap(),
        });
        reset_textareas(dt);
        dt.state = DumbTallyAppState::Main;
    }
}

fn reset_textareas(dt: &mut DumbTallyApp) {
    dt.new_counter = NewCounter::default();
}
