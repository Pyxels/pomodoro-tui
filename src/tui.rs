use std::io::{stdout, Bytes, Read, StdoutLock, Write};
use std::thread;
use std::time::Duration;

use termion::event::{parse_event, Event, Key};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{async_stdin, terminal_size, AsyncReader};

use crate::pomodoro::Pomodoro;

pub struct Tui<'a> {
    stdout: RawTerminal<StdoutLock<'a>>,
    stdin: Bytes<AsyncReader>,
    pomodoro: Pomodoro,
}

impl Tui<'_> {
    pub fn new() -> Tui<'static> {
        let stdout = stdout();
        let stdout = stdout.lock().into_raw_mode().unwrap();
        let stdin = async_stdin().bytes();

        let pomodoro = Pomodoro::new();
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
                Some(Ok(x)) => {
                    if let Ok(Event::Key(Key::Ctrl('c'))) = parse_event(x, &mut self.stdin) {
                        self.cleanup();
                        break;
                    }
                }
                _ => (),
            }

            let size = terminal_size().unwrap();
            self.pomodoro.tick();

            let state_string = self.get_state_string(size);
            let time_string = self.get_time_string(size);

            write!(
                self.stdout,
                "{}{}{}",
                state_string,
                time_string,
                termion::cursor::Hide,
            )
            .unwrap();
            self.stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn get_state_string(&self, size: (u16, u16)) -> String {
        let state_string = self.pomodoro.state();
        format!(
            "{}{}{}{}{}",
            termion::cursor::Goto(size.0 / 2 - (state_string.len() / 2) as u16, size.1 / 2 - 1),
            termion::clear::CurrentLine,
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
            "\n{}{}{}",
            termion::cursor::Goto(size.0 / 2 - (time_string.len() / 2) as u16, size.1 / 2),
            termion::clear::CurrentLine,
            time_string,
        )
    }

    fn cleanup(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }
}
