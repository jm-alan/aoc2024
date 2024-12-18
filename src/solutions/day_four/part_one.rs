use super::{XMASSES, XMAS_FOR, XMAS_REV};
use std::ops::{Index, IndexMut};

pub struct U32 {
  inner: [u8; 4],
  _alignment: [u32; 0],
}

impl U32 {
  #[inline(always)]
  pub fn new() -> Self {
    Self {
      inner: [0; 4],
      _alignment: [],
    }
  }
}

impl Index<usize> for U32 {
  type Output = u8;
  #[inline(always)]
  fn index<'ref_life>(&'ref_life self, idx: usize) -> &'ref_life u8 {
    &self.inner[idx]
  }
}

impl IndexMut<usize> for U32 {
  #[inline(always)]
  fn index_mut<'ref_life>(&'ref_life mut self, idx: usize) -> &'ref_life mut u8 {
    &mut self.inner[idx]
  }
}

pub fn solution() -> i64 {
  let mut sum = 0;
  let mut acomp = U32::new();

  let mut xr0;
  let mut xr1 = XMASSES[0];
  let mut xr2 = XMASSES[0];
  let mut xr3 = XMASSES[0];

  for row in 0..140 {
    for col in 0..140 {
      xr0 = XMASSES[row];
      acomp[0] = xr0[col];

      if row < 137 {
        xr1 = XMASSES[row + 1];
        xr2 = XMASSES[row + 2];
        xr3 = XMASSES[row + 3];
      }

      if col < 137 {
        acomp[1] = xr0[col + 1];
        acomp[2] = xr0[col + 2];
        acomp[3] = xr0[col + 3];
        sum += comp_tpun(&acomp);
      }
      if row < 137 {
        acomp[1] = xr1[col];
        acomp[2] = xr2[col];
        acomp[3] = xr3[col];
        sum += comp_tpun(&acomp);
      }
      if row < 137 && col < 137 {
        acomp[1] = xr1[col + 1];
        acomp[2] = xr2[col + 2];
        acomp[3] = xr3[col + 3];
        sum += comp_tpun(&acomp);
      }
      if row < 137 && col > 2 {
        acomp[1] = xr1[col - 1];
        acomp[2] = xr2[col - 2];
        acomp[3] = xr3[col - 3];
        sum += comp_tpun(&acomp);
      }
    }
  }

  sum
}

fn comp_tpun(val: &U32) -> i64 {
  let punned = unsafe { *(val as *const _ as *const u32) };

  (punned == XMAS_FOR || punned == XMAS_REV) as i64
}
