mod cli;
mod pomodoro;
mod tui;

extern crate termion;

use clap::Parser;

use crate::cli::Args;
use crate::tui::Tui;

fn main() {
    let Args {
        work: work_time,
        small_rest: small_rest_time,
        large_rest: large_rest_time,
    } = Args::parse();

    Tui::new(work_time, small_rest_time, large_rest_time).clear().start_loop();
}
