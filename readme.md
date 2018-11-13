### Description

This crate provides `Rust` timer types for measuring time in a program in different ways.
- `Timer` is a timer that counts up and knows how much time has passed since it was started.
- `EggTimer` is a timer that counts down from its set `Duration` and knows how much time it has left.
- `Stopwatch` is a timer that counts up and can be paused and resumed.

In addition to the timer types, a collection type, `TimedList`, is provided,
which associates each element with a `Duration` and only retains elements whose `Duration` has not elapsed.

### Usage

To use this crate, add this to your `Cargo.toml`:

```toml
[dependencies]
eggtimer = "0.3.0"
```

### Example

```rust
use eggtimer::Timer;

fn computation() {
    // Do some computation that takes some time.
}

fn main() {
    // Start the timer
    let timer = Timer::start();

    // Do a computation
    computation();

    // Check how long it took
    let elapsed = timer.elapsed();
    println!("The computation took {} seconds.", elapsed);
}
```
