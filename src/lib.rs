#[deny(missing_docs, unsafe_code)]
use std::time::Instant;

/// A simple timer that knows how long since it started
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timer {
    start: Instant,
}

impl Timer {
    /// Creates a new timer
    pub fn start() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }
    /// Gets the elapsed time in seconds
    pub fn elapsed(self) -> f64 {
        let duration = Instant::now().duration_since(self.start);
        duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) / 1_000_000_000.0
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::start()
    }
}

/// A timer that counts down
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EggTimer {
    timer: Timer,
    dur: f64,
}

impl EggTimer {
    /// Creates a new `EggTimer`
    pub fn set(seconds: f64) -> EggTimer {
        EggTimer {
            timer: Timer::start(),
            dur: seconds,
        }
    }
    /// Gets the time left in seconds
    pub fn time_left(self) -> f64 {
        self.dur - self.timer.elapsed()
    }
    /// Checks if the timer is ready
    pub fn is_ready(self) -> bool {
        self.time_left() <= 0.0
    }
}
