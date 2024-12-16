use super::{LEFT, RIGHT};

pub fn solution() -> i64 {
  let mut left_local = LEFT;
  let mut right_local = RIGHT;
  left_local.sort_unstable();
  right_local.sort_unstable();

  left_local
    .iter()
    .zip(right_local.iter())
    .fold(0, |sum, (l, r)| (*l - *r).abs() + sum)
}
