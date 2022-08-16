mod pomodoro;
mod tui;

extern crate termion;

use crate::tui::Tui;

fn main() {
    Tui::new().clear().start_loop();
}
