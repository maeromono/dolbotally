use ratatui::run;

use crate::dt::DumbTallyApp;

mod dt;

fn main() {
    println!("Hello, world!");
    run(|term| DumbTallyApp::default().run(term));
}
