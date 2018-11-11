#![deny(missing_docs, unsafe_code)]

//! This crate provides a veriety of timer types for measuring time in a program in different ways.

use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Converts a floating-point time value in seconds to a `Duration`
pub fn seconds_to_duration(seconds: f64) -> Duration {
    let whole = seconds as u64;
    let fract = (seconds.fract() * 1e9) as u32;
    Duration::new(whole, fract)
}

/// Converts a `Duration` to a floating-point time value
pub fn duration_to_seconds(duration: Duration) -> f64 {
    duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) / 1e9
}

/// A simple timer that knows how long since it started
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timer {
    start: Instant,
}

impl Timer {
    /// Creates a new `Timer`
    pub fn start() -> Timer {
        Timer {
            start: Instant::now(),
        }
    }
    /// Restarts the `Timer`
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
    /// Gets the elapsed time in seconds
    pub fn elapsed(self) -> f64 {
        let duration = Instant::now().duration_since(self.start);
        duration_to_seconds(duration)
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
    /// Resets the `EggTimer`
    pub fn reset(&mut self) {
        self.timer = Timer::start();
    }
    /// Gets the time left in seconds
    pub fn time_left(self) -> f64 {
        self.dur - self.timer.elapsed()
    }
    /// Checks if the timer is ready
    pub fn is_ready(self) -> bool {
        self.time_left() <= 0.0
    }
    /// Gets the number of seconds the `EggTimer` was originally set with
    pub fn max_time(&self) -> f64 {
        self.dur
    }
}

/// An alarm that calls a function when it is ready
pub struct Alarm<T>
where
    T: Send + 'static,
{
    handle: JoinHandle<T>,
    timer: EggTimer,
}

impl<T> Alarm<T>
where
    T: Send + 'static,
{
    /// Creates a new `Alarm`
    pub fn set<F>(seconds: f64, alarm: F) -> Alarm<T>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let timer = EggTimer::set(seconds);
        Alarm {
            handle: thread::spawn(move || {
                thread::sleep(seconds_to_duration(if seconds < 0.12 {
                    seconds * 0.9
                } else {
                    seconds - 0.1
                }));
                while !timer.is_ready() {}
                alarm()
            }),
            timer,
        }
    }
    /// Gets the time left in seconds
    pub fn time_left(&self) -> f64 {
        self.timer.time_left()
    }
    /// Checks if the timer is ready
    pub fn is_ready(&self) -> bool {
        self.timer.is_ready()
    }
    /// Blocks the current thread until the `Alarm` goes off
    pub fn join(self) -> thread::Result<T> {
        self.handle.join()
    }
}

/// An iterable list structure where each element has an associated duration.
/// When an element's duration has elapsed, the element is removed from the
/// list upon the next mutable function call.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct TimedList<T> {
    list: Vec<(EggTimer, T)>,
}

impl<T> TimedList<T> {
    /// Creates a new `TimedList`
    pub fn new() -> TimedList<T> {
        TimedList { list: Vec::new() }
    }
    /// Inserts and element into the list with the given duration
    pub fn insert(&mut self, element: T, duration: f64) {
        self.list.push((EggTimer::set(duration), element));
    }
    /// Forces the removal of all elements whose duration has elpased.
    /// This method does not need to be called manually.
    pub fn clean(&mut self) {
        self.list.retain(|(timer, _)| !timer.is_ready());
    }
    /// Removes all elements from the list
    pub fn clear(&mut self) {
        self.list.clear();
    }
    /// Iterates immutably through all elements.
    /// While this method does not remove timed-out elements,
    /// it does filter them out.
    /// If iteration takes sufficiently long, elements that
    /// may have been valid when iteration began may be skipped
    /// when they are atually iterated over.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.list.iter().filter_map(
            |(timer, elem)| {
                if timer.is_ready() {
                    None
                } else {
                    Some(elem)
                }
            },
        )
    }
    /// Iterates mutably through all elements.
    /// If iteration takes sufficiently long, elements that
    /// may have been valid when iteration began may be skipped
    /// when they are atually iterated over.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.clean();
        self.list.iter_mut().filter_map(
            |(timer, elem)| {
                if timer.is_ready() {
                    None
                } else {
                    Some(elem)
                }
            },
        )
    }
}

impl<T> IntoIterator for TimedList<T>
where
    T: 'static,
{
    type Item = T;
    type IntoIter = Box<Iterator<Item = T>>;
    fn into_iter(mut self) -> Self::IntoIter {
        self.clean();
        Box::new(self.list.into_iter().filter_map(
            |(timer, elem)| {
                if timer.is_ready() {
                    None
                } else {
                    Some(elem)
                }
            },
        ))
    }
}
