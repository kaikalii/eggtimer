#![deny(missing_docs, unsafe_code)]

//! This crate provides timer types for measuring time in a program in different ways.
//! - `Timer` is a timer that counts up and knows how much time has passed since it was started.
//! - `EggTimer` is a timer that counts down from its set `Duration` and knows how much time it has left.
//! - `Stopwatch` is a timer that counts up and can be paused and resumed.
//!
//! In addition to the timer types, a collection type, `TimedList`, is provided,
//! which associates each element with a `Duration` and only retains elements whose `Duration` has not elapsed.

use std::time::{Duration, Instant};

/// A trait for types that can be turned into a `Duration`
pub trait ToDuration {
    /// Convert the value into the `Duration`
    fn to_duration(&self) -> Duration;
}

impl ToDuration for Duration {
    fn to_duration(&self) -> Duration {
        *self
    }
}

impl ToDuration for f32 {
    fn to_duration(&self) -> Duration {
        if self < &0.0 {
            panic!("Attempted to convert negative f32 number to Duration");
        }
        let whole = *self as u64;
        let fract = (self.fract() * 1e9) as u32;
        Duration::new(whole, fract)
    }
}

impl ToDuration for f64 {
    fn to_duration(&self) -> Duration {
        if self < &0.0 {
            panic!("Attempted to convert negative f64 number to Duration");
        }
        let whole = *self as u64;
        let fract = (self.fract() * 1e9) as u32;
        Duration::new(whole, fract)
    }
}

impl ToDuration for u8 {
    fn to_duration(&self) -> Duration {
        Duration::new(u64::from(*self), 0)
    }
}

impl ToDuration for u16 {
    fn to_duration(&self) -> Duration {
        Duration::new(u64::from(*self), 0)
    }
}

impl ToDuration for u32 {
    fn to_duration(&self) -> Duration {
        Duration::new(u64::from(*self), 0)
    }
}

impl ToDuration for u64 {
    fn to_duration(&self) -> Duration {
        Duration::new(*self, 0)
    }
}

impl ToDuration for u128 {
    fn to_duration(&self) -> Duration {
        Duration::new(*self as u64, 0)
    }
}

impl ToDuration for usize {
    fn to_duration(&self) -> Duration {
        Duration::new(*self as u64, 0)
    }
}

/// A trait for types that can be created from a `Duration`
pub trait FromDuration {
    /// Create the value from a `Duration`
    fn from_duration(duration: Duration) -> Self;
}

impl FromDuration for Duration {
    fn from_duration(duration: Duration) -> Self {
        duration
    }
}

impl FromDuration for u64 {
    fn from_duration(duration: Duration) -> Self {
        duration.as_secs()
    }
}

impl FromDuration for u128 {
    fn from_duration(duration: Duration) -> Self {
        u128::from(duration.as_secs())
    }
}

impl FromDuration for usize {
    fn from_duration(duration: Duration) -> Self {
        duration.as_secs() as usize
    }
}

impl FromDuration for f32 {
    fn from_duration(duration: Duration) -> Self {
        duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1e9
    }
}

impl FromDuration for f64 {
    fn from_duration(duration: Duration) -> Self {
        duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) / 1e9
    }
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
    /// Gets the elapsed time as a floating-point number of seconds
    pub fn elapsed(self) -> f64 {
        f64::from_duration(self.duration())
    }
    /// Get the elapsed time as a `Duration`
    pub fn duration(self) -> Duration {
        Instant::now().duration_since(self.start)
    }
    /// Gets the `Instant` at which the `Timer` was started
    pub fn started_at(self) -> Instant {
        self.start
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer::start()
    }
}

/// A timer that counts down and knows when a `Duration` has elapsed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EggTimer {
    timer: Timer,
    duration: Duration,
}

impl EggTimer {
    /// Creates a new `EggTimer`
    pub fn set<D: ToDuration>(time: D) -> EggTimer {
        EggTimer {
            timer: Timer::start(),
            duration: time.to_duration(),
        }
    }
    /// Resets the `EggTimer`
    pub fn reset(&mut self) {
        self.timer = Timer::start();
    }
    /// Gets the time left as a `Duration`
    pub fn duration_left(&self) -> Option<Duration> {
        self.duration.checked_sub(self.timer.duration())
    }
    /// Gets the time left as a floating-point number of seconds
    pub fn seconds_left(&self) -> f64 {
        f64::from_duration(self.duration) - self.timer.elapsed()
    }
    /// Checks if the set `Duration` has elapsed
    pub fn is_ready(self) -> bool {
        self.duration_left().is_none()
    }
    /// Gets the time the `EggTimer` was originally set with as a `Duration`
    pub fn max_duration(&self) -> Duration {
        self.duration
    }
    /// Gets the time the `EggTimer` was originally set with as a floating-point number of seconds
    pub fn max_seconds(&self) -> f64 {
        f64::from_duration(self.max_duration())
    }
    /// Gets the elapsed time as a floating-point number of seconds
    pub fn elapsed(&self) -> f64 {
        self.timer.elapsed()
    }
    /// Get the elapsed time as a `Duration`
    pub fn duration(&self) -> Duration {
        self.timer.duration()
    }
    /// Gets the `Instant` at which the `EggTimer` was started
    pub fn started_at(&self) -> Instant {
        self.timer.started_at()
    }
    /// Gets the `Instant` at which the `EggTimer` will or did end
    pub fn ends_at(&self) -> Instant {
        self.timer.started_at() + self.duration
    }
}

/// A timer that can be paused and resumed.
///
/// The reported elapsed times do not include periods when the timer was paused
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stopwatch {
    last_start: Instant,
    prev_dur: Duration,
    paused: bool,
}

impl Stopwatch {
    /// Creates a new `Stopwatch` which immediately starts counting
    pub fn start() -> Stopwatch {
        Stopwatch {
            last_start: Instant::now(),
            prev_dur: 0u64.to_duration(),
            paused: false,
        }
    }
    /// Creates a new `Stopwatch` which starts paused
    pub fn start_paused() -> Stopwatch {
        Stopwatch {
            last_start: Instant::now(),
            prev_dur: 0u64.to_duration(),
            paused: true,
        }
    }
    /// Restarts the `Stopwatch` without pausing or resuming
    pub fn reset(&mut self) {
        self.last_start = Instant::now();
        self.prev_dur = 0u64.to_duration();
    }
    /// Gets the elapsed time as a floating-point number of seconds
    pub fn elapsed(&self) -> f64 {
        f64::from_duration(self.duration())
    }
    /// Gets the elapsed time as a `Duration`
    pub fn duration(&self) -> Duration {
        if self.paused {
            self.prev_dur
        } else {
            self.prev_dur + Instant::now().duration_since(self.last_start)
        }
    }
    /// Pauses the `Stopwatch`
    pub fn pause(&mut self) {
        if !self.paused {
            self.prev_dur += Instant::now().duration_since(self.last_start);
        }
    }
    /// Resumes the `Stopwatch`
    pub fn resume(&mut self) {
        if self.paused {
            self.last_start = Instant::now();
        }
    }
    /// Toggles whether the `Stopwatch` is paused or resumed
    pub fn toggle(&mut self) {
        if self.paused {
            self.pause();
        } else {
            self.resume();
        }
    }
    /// Gets the `Instant` at which the `Stopwatch` was last resumed
    pub fn started_at(&self) -> Instant {
        self.last_start
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Stopwatch::start()
    }
}

/// An iterable list structure where each element has an associated `Duration`.
///
/// When an element's `Duration` has elapsed, the element is removed from the
/// list upon the next mutable function call. Timed-out elements will never be iterated over.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct TimedList<T> {
    list: Vec<(EggTimer, T)>,
}

impl<T> TimedList<T> {
    /// Creates a new `TimedList`
    pub fn new() -> TimedList<T> {
        TimedList { list: Vec::new() }
    }
    /// Inserts an element into the list with the given number of floating-point seconds
    pub fn insert<D: ToDuration>(&mut self, element: T, time: D) {
        self.list.push((EggTimer::set(time), element));
    }
    /// Forces the removal of all elements whose `Duration` has elpased.
    /// This method does not need to be called manually unless you
    /// want to explicitely free the memory of timed-out elements immediately.
    pub fn clean(&mut self) {
        self.list.retain(|(timer, _)| !timer.is_ready());
    }
    /// Removes all elements from the list
    pub fn clear(&mut self) {
        self.list.clear();
    }
    /// Gets the number of elements in the list that have not timed out.
    pub fn len(&self) -> usize {
        self.iter().count()
    }
    /// Check if the list is empty or if all existing elements have timed out.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Retains elements in the list that match the predicate
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.list.retain(|(_, elem)| f(elem));
    }
    /// Iterates immutably through all elements.
    ///
    /// While this method does not remove timed-out elements,
    /// it does filter them out.
    /// If iteration takes sufficiently long, elements that
    /// may have been valid when iteration began may be skipped
    /// when they are actually iterated over.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
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
    ///
    /// If iteration takes sufficiently long, elements that
    /// may have been valid when iteration began may be skipped
    /// when they are actually iterated over.
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> {
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
    /// Iterates immutably through all elements and their timers.
    ///
    /// While this method does not remove timed-out elements,
    /// it does filter them out.
    /// If iteration takes sufficiently long, elements that
    /// may have been valid when iteration began may be skipped
    /// when they are actually iterated over.
    pub fn timer_iter(&self) -> impl DoubleEndedIterator<Item = (&T, EggTimer)> {
        self.list.iter().filter_map(|(timer, elem)| {
            if timer.is_ready() {
                None
            } else {
                Some((elem, *timer))
            }
        })
    }
    /// Iterates mutably through all elements.
    ///
    /// If iteration takes sufficiently long, elements that
    /// may have been valid when iteration began may be skipped
    /// when they are actually iterated over.
    pub fn timer_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (&mut T, EggTimer)> {
        self.clean();
        self.list.iter_mut().filter_map(|(timer, elem)| {
            if timer.is_ready() {
                None
            } else {
                Some((elem, *timer))
            }
        })
    }
}

impl<T, D> std::iter::FromIterator<(T, D)> for TimedList<T>
where
    D: ToDuration,
{
    fn from_iter<I: IntoIterator<Item = (T, D)>>(iter: I) -> Self {
        TimedList {
            list: iter
                .into_iter()
                .map(|(x, d)| (EggTimer::set(d), x))
                .collect(),
        }
    }
}

impl<T> IntoIterator for TimedList<T>
where
    T: 'static,
{
    type Item = T;
    type IntoIter = Box<DoubleEndedIterator<Item = T>>;
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

/**
Measure the amount of time the given function takes to execute

Time is measured in floating-point number of seconds

# Example
```
use eggtimer::measure;

let elapsed = measure(|| {
    for i in 0..1000 {
        println!("{}", i);
    }
});

println!("Printing all those numbers took {} seconds", elapsed);
```
*/
pub fn measure<F>(f: F) -> f64
where
    F: FnOnce(),
{
    let timer = Timer::start();
    f();
    timer.elapsed()
}
