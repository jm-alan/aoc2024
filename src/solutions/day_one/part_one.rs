use super::{LEFT, RIGHT};

pub fn solution() -> i32 {
  let mut left = LEFT.clone();
  let mut right = RIGHT.clone();

  left.sort_unstable();
  right.sort_unstable();

  left
    .iter()
    .zip(right.iter())
    .fold(0, |sum, (l, r)| (*l - *r).abs() + sum)
}
