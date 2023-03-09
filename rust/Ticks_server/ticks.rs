use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now(); // get the current time

    // simulate some work on the server
    for i in 0..10000000 {
        let x = i * 2;
    }

    let elapsed = start.elapsed(); // get the elapsed time since start
    let ticks = elapsed.as_micros(); // get the elapsed time in microseconds

    println!("Ticks: {}", ticks);
}
