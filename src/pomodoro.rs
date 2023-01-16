use std::{process::Command, time::SystemTime};

use crate::cli::PomodoroArgs;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Work(u8),
    SmallBreak(u8),
    LargeBreak,
    Overtime(Box<State>),
}

#[derive(Debug)]
pub struct Pomodoro {
    work_time: i64,
    small_rest_time: i64,
    large_rest_time: i64,
    state: State,
    start_time: SystemTime,
    send_notifications: bool,
    allow_continue: bool,
}

impl Pomodoro {
    pub fn new(args: PomodoroArgs) -> Pomodoro {
        Pomodoro {
            work_time: args.work * 60,
            small_rest_time: args.small_rest * 60,
            large_rest_time: args.large_rest * 60,
            state: State::Work(1),
            start_time: SystemTime::now(),
            send_notifications: args.notifications,
            allow_continue: args.allow_continue,
        }
    }

    pub fn tick(&mut self) {
        let mut notification: Option<String> = None;
        match self.state {
            State::Work(4) => {
                if self.seconds_passed() > self.work_time {
                    self.state = State::Overtime(Box::new(State::Work(4)));
                    notification = Some(String::from(
                        "A whole work cycle is done 💪, time for a well deserved large break! 🎉",
                    ));
                }
            }
            State::Work(x) => {
                if self.seconds_passed() > self.work_time {
                    self.state = State::Overtime(Box::new(State::Work(x)));
                    notification = Some(format!("Work {x} is done, take a short break. 鈴"));
                }
            }
            State::SmallBreak(x) => {
                if self.seconds_passed() > self.small_rest_time {
                    self.state = State::Overtime(Box::new(State::SmallBreak(x)));
                    notification = Some(format!(
                        "Small break {x} is done, lets get back to working. "
                    ));
                }
            }
            State::LargeBreak => {
                if self.seconds_passed() > self.large_rest_time {
                    self.state = State::Overtime(Box::new(State::LargeBreak));
                    notification = Some(String::from(
                        "Large break is over 😢. Lets do this again! 💪",
                    ));
                }
            }
            _ => {
                notification = None;
            }
        }
        if let Some(notification) = notification {
            if self.send_notifications {
                Command::new("notify-send")
                    .arg("Pomodoro 🍅")
                    .arg(notification)
                    .output()
                    .expect("Failed to send notification");
            }
        }
    }

    pub fn next(&mut self) {
        let state = match &self.state {
            State::Overtime(state) => &state,
            _ if self.allow_continue => &self.state,
            _ => return,
        };

        match state {
            State::Work(4) => {
                self.state = State::LargeBreak;
            }
            State::Work(x) => {
                self.state = State::SmallBreak(*x);
            }
            State::SmallBreak(x) => {
                self.state = State::Work(x + 1);
            }
            State::LargeBreak => {
                self.state = State::Work(1);
            }
            State::Overtime(_) => {
                unreachable!()
            }
        }
        self.start_time = SystemTime::now();
    }

    fn seconds_passed(&self) -> i64 {
        self.start_time.elapsed().unwrap().as_secs() as i64
    }

    pub fn seconds_remaining(&self) -> i64 {
        let mut state = &self.state;
        if let State::Overtime(x) = state {
            state = x;
        }
        match state {
            State::Work(_) => self.work_time - self.seconds_passed(),
            State::SmallBreak(_) => self.small_rest_time - self.seconds_passed(),
            State::LargeBreak => self.large_rest_time - self.seconds_passed(),
            State::Overtime(_) => unreachable!(),
        }
    }

    pub fn print_state(&self) -> String {
        let mut output = String::new();
        let mut state = &self.state;
        if let State::Overtime(x) = &self.state {
            output = String::from("Overtime: ");
            state = x;
        }
        format!(
            "{}{}",
            output,
            match state {
                State::Work(x) => format!("Work Nr. {}", x),
                State::SmallBreak(x) => format!("Small Break Nr. {}", x),
                State::LargeBreak => "Large Break".to_string(),
                _ => unreachable!(),
            }
        )
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }
}
