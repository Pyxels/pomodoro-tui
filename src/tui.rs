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
                Some(Ok(b'q')) => break,
                Some(Ok(x)) => {
                    if let Ok(Event::Key(Key::Ctrl('c'))) = parse_event(x, &mut self.stdin) {
                        break;
                    }
                }
                _ => (),
            }

            let size = terminal_size().unwrap();
            self.pomodoro.tick();

            let output_string = format!(
                "{} - Time left: {}s",
                self.pomodoro.state(),
                self.pomodoro.seconds_remaining(),
            );

            write!(
                self.stdout,
                "{}{}{}",
                termion::clear::CurrentLine,
                termion::cursor::Goto(size.0 / 2 - (output_string.len() / 2) as u16, size.1 / 2),
                output_string,
            )
            .unwrap();
            self.stdout.flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
}
