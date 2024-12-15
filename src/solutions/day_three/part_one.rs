use super::{digimul, CORRUPTED_MEMORY};

pub fn solution() -> i64 {
  let mut state: u8 = 0;
  let mut sum = 0;
  let mut left: [i64; 3] = [0; 3];
  let mut right: [i64; 3] = [0; 3];
  let mut lpos = 0;
  let mut rpos = 0;

  for ch in CORRUPTED_MEMORY.bytes() {
    match (ch, state) {
      (b'm', 0) => state = 1,
      (b'u', 1) => state = 2,
      (b'l', 2) => state = 3,
      (b'(', 3) => state = 4,
      (b'0'..=b'9', 4) => {
        state = 5;
        left[lpos] = (ch - 48) as i64;
        lpos += 1;
      },
      (b'0'..=b'9', 5) => {
        left[lpos] = (ch - 48) as i64;
        lpos += 1;
      },
      (b',', 5) => state = 6,
      ((b'0'..=b'9'), 6) => {
        state = 7;
        right[rpos] = (ch - 48) as i64;
        rpos += 1;
      },
      (b'0'..=b'9', 7) => {
        right[rpos] = (ch - 48) as i64;
        rpos += 1;
      },
      (b')', 7) => {
        state = 0;
        digimul(&mut sum, &left[0..lpos], &right[0..rpos]);
        lpos = 0;
        rpos = 0;
      },
      _ => {
        state = 0;
        lpos = 0;
        rpos = 0;
      },
    }
  }

  sum
}
