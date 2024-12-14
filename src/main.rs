use std::time::{Duration, Instant};

use solutions::{day_four, day_one, day_three, day_two};

mod solutions;

const ITERATIONS: usize = 10000;

fn store(averages: &mut Vec<Duration>, durations: &[Duration]) {
  averages.push(durations.iter().sum::<Duration>() / (ITERATIONS as u32));
}

fn main() {
  let mut durations = vec![Duration::new(0, 0); ITERATIONS];
  let mut then = Instant::now();
  let start = then;
  let mut averages = vec![];

  let mut local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_one::part_one::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  let mut local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 1, part 1: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_one::part_two::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 1, part 2: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_two::part_one::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 2, part 1: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_two::part_two::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 2, part 2: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_three::part_one::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 3, part 1: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_three::part_two::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 3, part 2: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_four::part_one::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  local_end = Instant::now();
  store(&mut averages, &durations);
  println!(
    "Day 4, part 1: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );

  local_start = Instant::now();
  for i in 0..ITERATIONS {
    then = Instant::now();
    day_four::part_two::solution();
    durations[i] = Instant::now().duration_since(then);
  }
  let fin = Instant::now();
  local_end = Instant::now();
  store(&mut averages, &durations);

  println!(
    "Day 4, part 2: {:?} ({:?} total)",
    averages.last().unwrap(),
    local_end.duration_since(local_start)
  );
  println!(
    "Total runtime for {ITERATIONS} runs of all ({} operations): {:?}",
    ITERATIONS * averages.len(),
    fin.duration_since(start)
  );
  // println!("Day 1, part 1: {}", day_one::part_one::solution());
  // println!("Day 1, part 2: {}", day_one::part_two::solution());
  // println!("Day 2, part 1: {}", day_two::part_one::solution());
  // println!("Day 2, part 2: {}", day_two::part_two::solution());
  // println!("Day 3, part 1: {}", day_three::part_one::solution());
  // println!("Day 3, part 2: {}", day_three::part_two::solution());
  // println!("Day 4, part 1: {}", day_four::part_one::solution());
  // println!("Day 4, part 2: {}", day_four::part_two::solution());
}
