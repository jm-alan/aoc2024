use super::{LEFT_MUTREF, RIGHT_MUTREF};

pub fn solution() -> i64 {
  unsafe {
    LEFT_MUTREF.sort_unstable();
    RIGHT_MUTREF.sort_unstable();
  }

  unsafe {
    LEFT_MUTREF
      .iter()
      .zip(RIGHT_MUTREF.iter())
      .fold(0, |sum, (l, r)| (*l - *r).abs() + sum)
  }
}
