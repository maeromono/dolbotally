use std::process::exit;

use ratatui::run;

use crate::{dt::{DumbTallyApp, DumbTallyAppState}, save::load_counters};

mod counter;
mod counterlist;
mod dt;
mod dtmain;
mod newcounter;
mod save;

fn main() {
    let mut dt = DumbTallyApp::default();
    dt.counters = load_counters().unwrap_or_default();
    if dt.counters.counters.is_empty() {
        dt.state = DumbTallyAppState::NewCounter;
    }
    if let Err(err) = run(|term| dt.run(term)) {
        eprintln!("!! Terminal error: {}", err);
        exit(1);
    }
}
