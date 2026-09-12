use crate::dt::{Counter, DumbTallyApp, DumbTallyAppState};
use crossterm::event::{KeyCode, KeyEvent};
use enum_iterator::{Sequence, next_cycle, previous_cycle};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::Paragraph,
};
use ratatui_textarea::TextArea;

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

impl Default for NewCounter {
    fn default() -> Self {
        Self {
            current_focus: CurrentFocus::Name,
            counter_name_textarea: TextArea::default(),
            counter_posadd_textarea: TextArea::new(vec![String::from('1')]),
            counter_negadd_textarea: TextArea::new(vec![String::from('1')]),
            counter_minval_textarea: TextArea::new(vec![String::from('0')]),
            counter_maxval_textarea: TextArea::new(vec![String::from("50")]),
            counter_posadd_valid: true,
            counter_negadd_valid: true,
            counter_minval_valid: true,
            counter_maxval_valid: true,
        }
    }
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
    let (direction, vert_constraint) = if area.width > 50 {
        (Direction::Horizontal, Constraint::Length(1))
    } else {
        (Direction::Vertical, Constraint::Length(2))
    };
    let newcounter_layout = Layout::vertical([vert_constraint; 5])
        .split(area.centered(Constraint::Max(75), Constraint::Percentage(30)));
    let field_layout = Layout::default()
        .direction(direction)
        .constraints([Constraint::Fill(1), Constraint::Fill(2)]);
    let name_field_layout = field_layout.clone().split(newcounter_layout[0]);
    let posadd_field_layout = field_layout.clone().split(newcounter_layout[1]);
    let negadd_field_layout = field_layout.clone().split(newcounter_layout[2]);
    let minval_field_layout = field_layout.clone().split(newcounter_layout[3]);
    let maxval_field_layout = field_layout.clone().split(newcounter_layout[4]);
    let focus = &dt.new_counter.current_focus;
    frame.render_widget(
        Paragraph::new("Counter name").style(focus_highlight(matches!(focus, CurrentFocus::Name))),
        name_field_layout[0],
    );
    frame.render_widget(&dt.new_counter.counter_name_textarea, name_field_layout[1]);
    frame.render_widget(
        Paragraph::new("Add amount").style(focus_highlight(matches!(focus, CurrentFocus::PosAdd))),
        posadd_field_layout[0],
    );
    frame.render_widget(
        &dt.new_counter.counter_posadd_textarea,
        posadd_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Sub amount").style(focus_highlight(matches!(focus, CurrentFocus::NegAdd))),
        negadd_field_layout[0],
    );
    frame.render_widget(
        &dt.new_counter.counter_negadd_textarea,
        negadd_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Min. value").style(focus_highlight(matches!(focus, CurrentFocus::MinVal))),
        minval_field_layout[0],
    );
    frame.render_widget(
        &dt.new_counter.counter_minval_textarea,
        minval_field_layout[1],
    );
    frame.render_widget(
        Paragraph::new("Max. value").style(focus_highlight(matches!(focus, CurrentFocus::MaxVal))),
        maxval_field_layout[0],
    );
    frame.render_widget(
        &dt.new_counter.counter_maxval_textarea,
        maxval_field_layout[1],
    );
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            dt.state = DumbTallyAppState::Main;
            reset_textareas(dt);
        }
        KeyCode::Tab => dt.new_counter.current_focus = next_cycle(&dt.new_counter.current_focus),
        KeyCode::BackTab => {
            dt.new_counter.current_focus = previous_cycle(&dt.new_counter.current_focus)
        }
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

fn focus_highlight(focused: bool) -> Style {
    if focused {
        Style::default().bold().reversed()
    } else {
        Style::default()
    }
}

fn validate_ints(textarea: &mut TextArea) -> bool {
    if let Err(_err) = textarea.lines()[0].parse::<i64>() {
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
        dt.counters.counters.push(Counter {
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
        dt.save();
        reset_textareas(dt);
        dt.state = DumbTallyAppState::Main;
    }
}

fn reset_textareas(dt: &mut DumbTallyApp) {
    dt.new_counter = NewCounter::default();
}
