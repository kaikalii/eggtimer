### Description

This crate provides Rust timer types for measuring time in a program in different ways.
- `Elapsed` is a timer that counts up and knows how much time has passed since it was started.
- `Timer` is a timer that counts down from its set `Duration` and knows how much time it has left.
- `Stopwatch` is a timer that counts up and can be paused and resumed.

In addition to the timer types, a collection type, `TimedList`, is provided,
which associates each element with a `Duration` and only retains elements whose `Duration` has not elapsed.

### Usage

To use this crate, add this to your `Cargo.toml`:

```toml
[dependencies]
eggtimer = "0.6.0"
```

### Example

```rust
use eggtimer::Elapsed;

fn computation() {
    // Do some computation that takes some time.
}

fn main() {
    // Start the timer
    let timer = Elapsed::start();

    // Do a computation
    computation();

    // Check how long it took
    let elapsed = timer.seconds();
    println!("The computation took {} seconds.", elapsed);
}
```
