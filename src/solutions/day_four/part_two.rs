use super::{MASX_MB, MASX_ML, MASX_MR, MASX_MT, XMASSES};

pub fn solution() -> u16 {
  let mut sum = 0;

  for row in 0..138 {
    for col in 0..138 {
      let x = [
        XMASSES[row][col],
        XMASSES[row][col + 2],
        XMASSES[row + 1][col + 1],
        XMASSES[row + 2][col],
        XMASSES[row + 2][col + 2],
      ];
      sum += (x == MASX_MT || x == MASX_MR || x == MASX_MB || x == MASX_ML) as u16;
    }
  }

  sum
}
