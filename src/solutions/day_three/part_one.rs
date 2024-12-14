use super::CORRUPTED_MEMORY;

pub fn solution() -> u32 {
  let mut state: u8 = 0;
  let mut sum = 0;
  let mut left: [u32; 3] = [0; 3];
  let mut right: [u32; 3] = [0; 3];
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
        left[lpos] = (ch - 48) as u32;
        lpos += 1;
      },
      (b'0'..=b'9', 5) => {
        left[lpos] = (ch - 48) as u32;
        lpos += 1;
      },
      (b',', 5) => state = 6,
      ((b'0'..=b'9'), 6) => {
        state = 7;
        right[rpos] = (ch - 48) as u32;
        rpos += 1;
      },
      (b'0'..=b'9', 7) => {
        right[rpos] = (ch - 48) as u32;
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

#[inline(always)]
fn digimul(sum: &mut u32, left: &[u32], right: &[u32]) {
  *sum += binconv(left) * binconv(right);
}

#[inline(always)]
fn binconv(digital: &[u32]) -> u32 {
  let mut val = 0u32;
  *digital
    .iter()
    .rev()
    .enumerate()
    .fold(&mut val, |sum, (idx, val)| {
      *sum += (*val) * 10u32.pow(idx as u32);
      sum
    })
}
