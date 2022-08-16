use std::io::{stdout, Bytes, Read, StdoutLock, Write};
use std::thread;
use std::time::Duration;

use termion::event::{parse_event, Event, Key};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, terminal_size, AsyncReader};

use crate::pomodoro::{Pomodoro, State};

pub struct Tui<'a> {
    stdout: RawTerminal<StdoutLock<'a>>,
    stdin: Bytes<AsyncReader>,
    pomodoro: Pomodoro,
}

impl Tui<'_> {
    pub fn new(work_time: i64, small_rest_time: i64, large_rest_time: i64) -> Tui<'static> {
        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = async_stdin().bytes();

        let pomodoro = Pomodoro::new(work_time, small_rest_time, large_rest_time);
        Tui {
            stdout,
            stdin,
            pomodoro,
        }
    }

    pub fn clear(mut self) -> Self {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        self
    }

    pub fn start_loop(mut self) {
        loop {
            let b = self.stdin.next();
            match b {
                Some(Ok(b'q')) => {
                    self.cleanup();
                    break;
                }
                Some(Ok(x)) => match parse_event(x, &mut self.stdin) {
                    Ok(Event::Key(Key::Ctrl('c'))) => {
                        self.cleanup();
                        break;
                    }
                    Ok(Event::Key(Key::Char('n'))) => {
                        self.pomodoro.next();
                    }
                    _ => (),
                },
                _ => (),
            }

            let size = terminal_size().unwrap();
            self.pomodoro.tick();

            let state_string = self.get_state_string(size);
            let time_string = self.get_time_string(size);
            let color_string = self.get_color_string();
            let footer_string = Self::get_footer(size);

            write!(
                self.stdout,
                "{}{}{}{}{}{}",
                color_string,
                state_string,
                color_string,
                time_string,
                footer_string,
                termion::cursor::Hide,
            )
            .unwrap();
            self.stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn get_state_string(&self, size: (u16, u16)) -> String {
        let state_string = self.pomodoro.print_state();
        format!(
            "{}{}{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(size.0 / 2 - (state_string.len() / 2) as u16, size.1 / 2 - 1),
            termion::style::Bold,
            state_string,
            termion::style::Reset,
        )
    }

    fn get_time_string(&self, size: (u16, u16)) -> String {
        let seconds_remaining = self.pomodoro.seconds_remaining();
        let time_string = match seconds_remaining {
            x if x < 60 => format!("Time left: {}s", x),
            x => format!("Time left: {}m", x / 60),
        };
        format!(
            "\n{}{}",
            termion::cursor::Goto(size.0 / 2 - (time_string.len() / 2) as u16, size.1 / 2),
            time_string,
        )
    }

    fn get_color_string(&self) -> String {
        match self.pomodoro.state() {
            State::Overtime(_) => termion::color::Bg(termion::color::Red).to_string(),
            _ => termion::color::Bg(termion::color::Reset).to_string(),
        }
    }

    fn get_footer(size: (u16, u16)) -> String {
        let footer = String::from("press 'n' to continue - press 'q' to quit");
        format!(
            "{}{}{}{}",
            termion::cursor::Goto(size.0 / 2 - (footer.len() / 2) as u16, size.1),
            termion::style::Italic,
            footer,
            termion::style::Reset,
        )
    }

    fn cleanup(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            termion::color::Bg(termion::color::Reset),
            termion::clear::All,
            termion::cursor::Show
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}
