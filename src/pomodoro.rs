use std::time::SystemTime;

const WORK_TIME: i64 = 25 * 60;
const SMALL_REST_TIME: i64 = 5 * 60;
const LARGE_REST_TIME: i64 = 35 * 60;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Work(u8),
    SmallBreak(u8),
    LargeBreak,
    Overtime(Box<State>),
}

#[derive(Debug)]
pub struct Pomodoro {
    state: State,
    start_time: SystemTime,
}

impl Pomodoro {
    pub fn new() -> Pomodoro {
        Pomodoro {
            state: State::Work(1),
            start_time: SystemTime::now(),
        }
    }

    pub fn tick(&mut self) {
        match self.state {
            State::Work(4) => {
                if self.seconds_passed() > WORK_TIME {
                    self.state = State::Overtime(Box::new(State::Work(4)));
                }
            }
            State::Work(x) => {
                if self.seconds_passed() > WORK_TIME {
                    self.state = State::Overtime(Box::new(State::Work(x)));
                }
            }
            State::SmallBreak(x) => {
                if self.seconds_passed() > SMALL_REST_TIME {
                    self.state = State::Overtime(Box::new(State::SmallBreak(x)));
                }
            }
            State::LargeBreak => {
                if self.seconds_passed() > LARGE_REST_TIME {
                    self.state = State::Overtime(Box::new(State::LargeBreak));
                }
            }
            _ => (),
        }
    }

    pub fn next(&mut self) {
        if let State::Overtime(state) = &self.state {
            match **state {
                State::Work(4) => {
                    self.state = State::LargeBreak;
                }
                State::Work(x) => {
                    self.state = State::SmallBreak(x);
                }
                State::SmallBreak(x) => {
                    self.state = State::Work(x+1);
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
            State::Work(_) => WORK_TIME - self.seconds_passed(),
            State::SmallBreak(_) => SMALL_REST_TIME - self.seconds_passed(),
            State::LargeBreak => LARGE_REST_TIME - self.seconds_passed(),
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
