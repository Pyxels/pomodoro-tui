use std::time::SystemTime;

const WORK_TIME: u64 = 25 * 60;
const SMALL_REST_TIME: u64 = 5 * 60;
const LARGE_REST_TIME: u64 = 35 * 60;

#[derive(Debug)]
enum State {
    Work(u8),
    SmallBreak(u8),
    LargeBreak,
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
                // 25 * 60
                if self.seconds_passed() > WORK_TIME {
                    self.state = State::LargeBreak;
                    self.start_time = SystemTime::now();
                }
            }
            State::Work(x) => {
                // 25 * 60
                if self.seconds_passed() > WORK_TIME {
                    self.state = State::SmallBreak(x);
                    self.start_time = SystemTime::now();
                }
            }
            State::SmallBreak(x) => {
                // 5 * 60
                if self.seconds_passed() > SMALL_REST_TIME {
                    self.state = State::Work(x + 1);
                    self.start_time = SystemTime::now();
                }
            }
            State::LargeBreak => {
                // 35 * 60
                if self.seconds_passed() > LARGE_REST_TIME {
                    self.state = State::Work(1);
                    self.start_time = SystemTime::now();
                }
            }
        }
    }

    fn seconds_passed(&self) -> u64 {
        self.start_time.elapsed().unwrap().as_secs()
    }

    pub fn seconds_remaining(&self) -> u64 {
        match self.state {
            State::Work(_) => WORK_TIME - self.seconds_passed(),
            State::SmallBreak(_) => SMALL_REST_TIME - self.seconds_passed(),
            State::LargeBreak => LARGE_REST_TIME - self.seconds_passed(),
        }
    }

    pub fn state(&self) -> String {
        match self.state {
            State::Work(x) => format!("Work Nr. {}", x),
            State::SmallBreak(x) => format!("Small Break Nr. {}", x),
            State::LargeBreak => "Large Break".to_string(),
        }
    }
}
