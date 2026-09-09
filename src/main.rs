use ratatui::run;

use crate::dt::DumbTallyApp;

mod counter;
mod dt;
mod header;

fn main() {
    println!("Hello, world!");
    run(|term| DumbTallyApp::default().run(term));
}
