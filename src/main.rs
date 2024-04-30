mod cli;
mod pomodoro;
mod tui;

extern crate termion;

use clap::Parser;

use crate::cli::PomodoroArgs;
use crate::tui::Tui;

fn main() {
    let args = PomodoroArgs::parse();

    Tui::new(args).clear().start_loop();
}
