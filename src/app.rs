use notify_rust::Notification;
use std::time::{Duration, Instant};
#[derive(Clone, Copy)]

pub enum Mode {
    Work,
    Break,
}

impl Mode {
    fn send_notification(self) {
        let (title, body) = match self {
            Mode::Work => ("Pomodoro", "Time to focus!"),
            Mode::Break => ("Pomodoro", "Take a break!"),
        };

        let _ = Notification::new().summary(title).body(body).show();
    }
}

pub struct App {
    pub(crate) mode: Mode,
    pub(crate) running: bool,
    pub(crate) remaining: Duration,
    pub(crate) last_tick: Instant,
    pub(crate) break_time: Duration,
    pub(crate) work_time: Duration,
}

impl App {
    pub fn new(work_time: u64, break_time: u64) -> Self {
        Self {
            mode: Mode::Work,
            running: false,
            remaining: Duration::from_secs(work_time * 60),
            last_tick: Instant::now(),
            work_time: Duration::from_secs(work_time * 60),
            break_time: Duration::from_secs(break_time * 60),
        }
    }

    pub fn toggle(&mut self) {
        self.running = !self.running;
        self.last_tick = Instant::now();
    }

    pub fn reset_to_work(&mut self) {
        self.running = false;
        self.set_to_work();
    }

    pub fn reset_to_break(&mut self) {
        self.running = false;
        self.set_to_break();
    }
    pub fn switch_mode(&mut self) {
        match self.mode {
            Mode::Work => self.reset_to_break(),
            Mode::Break => self.reset_to_work(),
        }
    }

    pub fn set_to_work(&mut self) {
        self.mode = Mode::Work;
        self.remaining = self.work_time;
    }

    pub fn set_to_break(&mut self) {
        self.mode = Mode::Break;
        self.remaining = self.break_time;
    }

    pub fn tick(&mut self) {
        if !self.running {
            return;
        }

        let now = Instant::now();
        let elapsed = now - self.last_tick;
        self.last_tick = now;

        if self.remaining > elapsed {
            self.remaining -= elapsed;
        } else {
            match self.mode {
                Mode::Work => {
                    self.set_to_break();
                }
                Mode::Break => {
                    self.set_to_work();
                }
            };
            self.mode.send_notification();
        }
    }
}
