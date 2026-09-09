use std::process::exit;

use ratatui::run;

use crate::dt::DumbTallyApp;

mod counter;
mod counterlist;
mod dt;
mod header;
mod newcounter;

fn main() {
    if let Err(err) = run(|term| DumbTallyApp::default().run(term)) {
        eprintln!("Terminal error: {}", err);
        exit(1);
    }
}
