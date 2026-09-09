use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, layout::Constraint, style::Stylize, widgets::Paragraph};

#[derive(Debug)]
pub struct DumbTallyApp {
    state: DumbTallyAppState,
    counters: Vec<Counter>
}

impl Default for DumbTallyApp {
    fn default() -> Self {
        Self { state: DumbTallyAppState::Main, counters: vec![] }
    }
}

#[derive(Debug)]
pub struct Counter {
    name: String,
    value: i64,
    pos_add: i64,
    neg_add: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DumbTallyAppState {
    Main,
    CounterAdd,
    Quit,
}

impl DumbTallyApp {
    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while self.state != DumbTallyAppState::Quit {
            term.draw(|frame| self.render(frame))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keys(&key),
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        if area.width < 40 || area.height < 25 {
            // vv terminal too small text
            let ttst = Paragraph::new(format!(
                "Terminal too small\nRequired 40x25\nCurrent {}x{}",
                area.width, area.height
            ))
            .red()
            .centered();
            frame.render_widget(ttst, area.centered(Constraint::Length(15), Constraint::Length(3)));
        } else {
            println!("hello");
        }
    }

    fn handle_keys(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.state = DumbTallyAppState::Quit,
            _ => {}
        }
    }
}
