use super::{MMASS, MSAMS, SMASM, SSAMM, XMASSES};
use std::ops::{Index, IndexMut};

struct U64 {
  inner: [u8; 5],
  _alignment: [u64; 0],
}

impl U64 {
  #[inline(always)]
  pub fn new() -> Self {
    Self {
      inner: [0; 5],
      _alignment: [],
    }
  }
}

impl Index<usize> for U64 {
  type Output = u8;
  #[inline(always)]
  fn index<'ref_life>(&'ref_life self, idx: usize) -> &'ref_life u8 {
    &self.inner[idx]
  }
}

impl IndexMut<usize> for U64 {
  #[inline(always)]
  fn index_mut<'ref_life>(&'ref_life mut self, idx: usize) -> &'ref_life mut u8 {
    &mut self.inner[idx]
  }
}

pub fn solution() -> i64 {
  let mut sum = 0;
  let mut comp = U64::new();
  let mut xr0;
  let mut xr2;

  for row in 0..138 {
    for col in 0..138 {
      xr0 = XMASSES[row];
      xr2 = XMASSES[row + 2];
      comp[0] = xr0[col];
      comp[1] = xr0[col + 2];
      comp[2] = (&(XMASSES[row + 1]))[col + 1];
      comp[3] = xr2[col];
      comp[4] = xr2[col + 2];

      sum += comp_part_two(&comp);
    }
  }

  sum
}

fn comp_part_two(val: &U64) -> i64 {
  let punned = unsafe { *(val as *const _ as *const u64) };

  (punned == MMASS || punned == MSAMS || punned == SMASM || punned == SSAMM) as i64
}
