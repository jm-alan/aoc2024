use std::time::{Duration, Instant};

use crate::ITERATIONS;

static mut DURATIONS: [Duration; ITERATIONS] = [Duration::new(0, 0); ITERATIONS];
static mut AVERAGES: [Duration; ITERATIONS] = [Duration::new(0, 0); ITERATIONS];
static mut RESULTS: [i64; ITERATIONS] = [0; ITERATIONS];
static mut LOCAL_DURATION: Duration = Duration::new(0, 0);
static mut LOCAL_THEN: Option<Instant> = None;
static mut LOCAL_START: Option<Instant> = None;
static mut NEXT_AVERAGE: usize = 0;

unsafe fn store() {
  AVERAGES[NEXT_AVERAGE] = DURATIONS.iter().sum::<Duration>() / (ITERATIONS as u32);
}

pub unsafe fn stopwatch(some_fn: fn() -> i64, title: &str) {
  let start = Instant::now();
  let mut then;
  for i in 0..ITERATIONS {
    then = Instant::now();
    RESULTS[i] = some_fn();
    DURATIONS[i] = Instant::now().duration_since(then);
  }
  LOCAL_DURATION = Instant::now().duration_since(start);
  store();
  print!(
    "{title}: {:.6?} ({:.6?} total) ",
    AVERAGES[NEXT_AVERAGE], LOCAL_DURATION
  );
  println!("Sanity check: {}", RESULTS[0]);
}
