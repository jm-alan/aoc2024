use std::collections::HashMap;

use super::{LEFT, RIGHT};

pub fn solution() -> u64 {
  let mut right_freq = HashMap::new();
  RIGHT.iter().for_each(|val| {
    if let Some(freq) = right_freq.get_mut(val) {
      *freq += 1;
    } else {
      right_freq.insert(val, 1);
    }
  });

  let mut sum = 0;

  for val in LEFT.iter() {
    if right_freq.contains_key(val) {
      sum += (right_freq[val] * val) as u64;
    }
  }

  sum
}
