mod cli;
mod pomodoro;
mod tui;

extern crate termion;

use clap::Parser;

use crate::cli::Args;
use crate::tui::Tui;

fn main() {
    let Args {
        work,
        small_rest,
        large_rest,
        notifications,
        allow_continue,
    } = Args::parse();

    Tui::new(work, small_rest, large_rest, notifications, allow_continue)
        .clear()
        .start_loop();
}
