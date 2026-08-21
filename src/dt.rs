use std::io::Result;

use ratatui::DefaultTerminal;

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
            println!("fuck you");
        }
        Ok(())
    }
}
