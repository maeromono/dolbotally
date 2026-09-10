use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};
use ratatui_textarea::TextArea;
use crate::dt::DumbTallyApp;

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

#[derive(Default)]
pub enum CurrentFocus {
    #[default]
    Name,
    PosAdd,
    NegAdd,
    MinVal,
    MaxVal,
}

pub fn render_countermake_screen(dt: &mut DumbTallyApp, frame: &mut Frame, area: Rect) {
    dt.new_counter.counter_name_textarea.set_placeholder_text("Counter name");
    frame.render_widget(&dt.new_counter.counter_name_textarea, area);
}

pub fn handle_keys(dt: &mut DumbTallyApp, key: &KeyEvent) {
    if dt.new_counter.counter_name_textarea.input_without_shortcuts(*key) {

    }
}

fn reset_textareas(dt: &mut DumbTallyApp) {
    dt.new_counter = NewCounter::default();
}
