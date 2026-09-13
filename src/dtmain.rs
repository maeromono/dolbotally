use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Paragraph,
};

use crate::dt::{DumbTallyApp, DumbTallyAppState};

pub fn render_header(dt: &DumbTallyApp, frame: &mut Frame, area: Rect) {
    let header = Layout::horizontal([Constraint::Length(18), Constraint::Fill(2)]).split(area);
    let name_and_ver = Paragraph::new("dolbotally v0.3.0").bold();
    let wm = if dt.state == DumbTallyAppState::Main {
        Paragraph::new(dt.cached_counter_name.clone()).right_aligned()
    } else {
        Paragraph::new("enmkdev.neocities.org").right_aligned()
    };
    frame.render_widget(name_and_ver, header[0]);
    frame.render_widget(wm, header[1]);
}

pub fn render_footer(dt: &DumbTallyApp, frame: &mut Frame, area: Rect) {
    let bind_help_string = match dt.state {
        DumbTallyAppState::Main => "Esc list, - subtract, = add, n new",
        DumbTallyAppState::CounterPick => "Esc back, j/k navigate, Enter confirm",
        DumbTallyAppState::CounterEdit => "Esc cancel, Tab cycle, Enter confirm",
        _ => "",
    };
    let bind_help = Paragraph::new(bind_help_string).centered();
    frame.render_widget(bind_help, area);
}
