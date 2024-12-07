use super::{XMASSES, XMAS_FOR, XMAS_REV};

pub fn solution() -> u16 {
  let mut sum = 0;

  for row in 0..140 {
    for col in 0..140 {
      match (row, col) {
        (0..137, 3..137) => {
          let rrow = [
            XMASSES[row][col],
            XMASSES[row][col + 1],
            XMASSES[row][col + 2],
            XMASSES[row][col + 3],
          ];
          let rz = [
            XMASSES[row][col],
            XMASSES[row + 1][col + 1],
            XMASSES[row + 2][col + 2],
            XMASSES[row + 3][col + 3],
          ];
          let dcol = [
            XMASSES[row][col],
            XMASSES[row + 1][col],
            XMASSES[row + 2][col],
            XMASSES[row + 3][col],
          ];
          let lz = [
            XMASSES[row][col],
            XMASSES[row + 1][col - 1],
            XMASSES[row + 2][col - 2],
            XMASSES[row + 3][col - 3],
          ];
          sum += (rrow == XMAS_FOR || rrow == XMAS_REV) as u16;
          sum += (rz == XMAS_FOR || rz == XMAS_REV) as u16;
          sum += (dcol == XMAS_FOR || dcol == XMAS_REV) as u16;
          sum += (lz == XMAS_FOR || lz == XMAS_REV) as u16;
        },
        (0..137, 0..3) => {
          let rrow = [
            XMASSES[row][col],
            XMASSES[row][col + 1],
            XMASSES[row][col + 2],
            XMASSES[row][col + 3],
          ];
          let rz = [
            XMASSES[row][col],
            XMASSES[row + 1][col + 1],
            XMASSES[row + 2][col + 2],
            XMASSES[row + 3][col + 3],
          ];
          let dcol = [
            XMASSES[row][col],
            XMASSES[row + 1][col],
            XMASSES[row + 2][col],
            XMASSES[row + 3][col],
          ];
          sum += (rrow == XMAS_FOR || rrow == XMAS_REV) as u16;
          sum += (rz == XMAS_FOR || rz == XMAS_REV) as u16;
          sum += (dcol == XMAS_FOR || dcol == XMAS_REV) as u16;
        },
        (0..137, 137..140) => {
          let dcol = [
            XMASSES[row][col],
            XMASSES[row + 1][col],
            XMASSES[row + 2][col],
            XMASSES[row + 3][col],
          ];
          let lz = [
            XMASSES[row][col],
            XMASSES[row + 1][col - 1],
            XMASSES[row + 2][col - 2],
            XMASSES[row + 3][col - 3],
          ];
          sum += (dcol == XMAS_FOR || dcol == XMAS_REV) as u16;
          sum += (lz == XMAS_FOR || lz == XMAS_REV) as u16;
        },
        (137..140, 0..137) => {
          let rrow = [
            XMASSES[row][col],
            XMASSES[row][col + 1],
            XMASSES[row][col + 2],
            XMASSES[row][col + 3],
          ];
          sum += (rrow == XMAS_FOR || rrow == XMAS_REV) as u16;
        },
        _ => {},
      }
    }
  }

  sum
}
