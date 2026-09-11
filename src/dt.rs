use std::io::Result;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::{ListState, Paragraph},
};
use serde::{Deserialize, Serialize};

use crate::{
    counter::{self, render_counter},
    counterlist::{self, render_counter_list},
    dtmain::{render_footer, render_header},
    newcounter::{self, NewCounter, render_countermake_screen},
    save::{DolbotallyCounters, save_counters},
};

pub struct DumbTallyApp {
    pub state: DumbTallyAppState,
    pub counters: DolbotallyCounters,
    pub counter_list: ListState,
    pub new_counter: NewCounter,
}

impl Default for DumbTallyApp {
    fn default() -> Self {
        Self {
            state: DumbTallyAppState::Main,
            counters: DolbotallyCounters::default(),
            counter_list: ListState::default().with_selected(Some(0)),
            new_counter: NewCounter::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub name: String,
    pub value: i64,
    pub pos_add: i64,
    pub neg_add: i64,
    pub min_val: i64,
    pub max_val: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DumbTallyAppState {
    Main,
    CounterPick,
    NewCounter,
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
        self.save();
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
            frame.render_widget(
                ttst,
                area.centered(Constraint::Length(18), Constraint::Length(3)),
            );
        } else {
            let main_layout = Layout::vertical([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(area.centered_horizontally(Constraint::Max(100)));
            render_header(frame, main_layout[0]);
            render_footer(&self, frame, main_layout[2]);
            match self.state {
                DumbTallyAppState::Main => render_counter(self, frame, main_layout[1]),
                DumbTallyAppState::CounterPick => render_counter_list(self, frame, main_layout[1]),
                DumbTallyAppState::NewCounter => {
                    render_countermake_screen(self, frame, main_layout[1])
                }
                _ => todo!(),
            }
        }
    }

    fn handle_keys(&mut self, key: &KeyEvent) {
        match self.state {
            DumbTallyAppState::Main => counter::handle_keys(self, key),
            DumbTallyAppState::CounterPick => counterlist::handle_keys(self, key),
            DumbTallyAppState::NewCounter => newcounter::handle_keys(self, key),
            _ => {}
        }
    }

    pub fn save(&self) {
        if let Err(err) = save_counters(&self.counters) {
            eprintln!("!! Failed to save counters. Details: {}", err);
        }
    }
}
