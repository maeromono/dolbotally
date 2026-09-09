use ratatui::{Frame, layout::{Constraint, Layout, Rect}, style::Stylize, widgets::Paragraph};

pub fn render_header(frame: &mut Frame, area: Rect) {
    let header = Layout::horizontal([Constraint::Length(18), Constraint::Fill(2)]).split(area);
    let name_and_ver = Paragraph::new("dolbotally v0.1.0").bold();
    let wm = Paragraph::new("enmkdev.neocities.org").right_aligned();
    frame.render_widget(name_and_ver, header[0]);
    frame.render_widget(wm, header[1]);
}
