mod solutions;
mod timing;

use solutions::{day_five, day_four, day_one, day_three, day_two};
use timing::stopwatch;

pub const ITERATIONS: usize = 100000;

fn main() {
  unsafe {
    stopwatch(day_one::part_one::solution, "Day 1 part 1");
    stopwatch(day_one::part_two::solution, "Day 1 part 2");
    stopwatch(day_two::part_one::solution, "Day 2 part 1");
    stopwatch(day_two::part_two::solution, "Day 2 part 2");
    stopwatch(day_three::part_one::solution, "Day 3 part 1");
    stopwatch(day_three::part_two::solution, "Day 3 part 2");
    stopwatch(day_four::part_one::solution, "Day 4 part 1");
    stopwatch(day_four::part_two::solution, "Day 4 part 2");
    stopwatch(day_five::part_one::solution, "Day 5 part 1");
  }

  // println!("Day 1, part 1: {}", day_one::part_one::solution());
  // println!("Day 1, part 2: {}", day_one::part_two::solution());
  // println!("Day 2, part 1: {}", day_two::part_one::solution());
  // println!("Day 2, part 2: {}", day_two::part_two::solution());
  // println!("Day 3, part 1: {}", day_three::part_one::solution());
  // println!("Day 3, part 2: {}", day_three::part_two::solution());
  // println!("Day 4, part 1: {}", day_four::part_one::solution());
  // println!("Day 4, part 2: {}", day_four::part_two::solution());
}
