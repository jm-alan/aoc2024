use solutions::{day_four, day_one, day_three, day_two};

mod solutions;

fn main() {
  println!("Day 1, part 1: {}", day_one::part_one::solution());
  println!("Day 1, part 2: {}", day_one::part_two::solution());
  println!("Day 2, part 1: {}", day_two::part_one::solution());
  println!("Day 2, part 2: {}", day_two::part_two::solution());
  println!("Day 3, part 1: {}", day_three::part_one::solution());
  println!("Day 3, part 2: {}", day_three::part_two::solution());
  println!("Day 4, part 1: {}", day_four::part_one::solution());
  println!("Day 4, part 1: {}", day_four::part_two::solution());
}
