use super::{LEFT_NOREF, RIGHT_NOREF};

pub fn solution() -> i64 {
  let mut freq_map = [0; 99999];
  RIGHT_NOREF.iter().for_each(|val| {
    freq_map[unsafe { *(val as *const _ as *const usize) }] += 1;
  });

  let mut sum = 0;

  for val in LEFT_NOREF.iter() {
    sum += val * freq_map[unsafe { *(val as *const _ as *const usize) }];
  }

  sum
}
