use super::CORRUPTED_MEMORY;

pub fn solution() -> u128 {
  let mut state: u8 = 0;
  let mut sum = 0;
  let mut lbuf = "".to_string();
  let mut rbuf = "".to_string();

  for ch in CORRUPTED_MEMORY.chars() {
    match (ch, state) {
      ('m', 0) => state = 1,
      ('u', 1) => state = 2,
      ('l', 2) => state = 3,
      ('(', 3) => state = 4,
      ('0'..='9', 4) => {
        state = 5;
        lbuf.push(ch);
      },
      ('0'..='9', 5) => lbuf.push(ch),
      (',', 5) => state = 6,
      ('0'..='9', 6) => {
        state = 7;
        rbuf.push(ch);
      },
      ('0'..='9', 7) => rbuf.push(ch),
      (')', 7) => {
        state = 0;
        sum += lbuf.parse::<u128>().unwrap() * rbuf.parse::<u128>().unwrap();
        lbuf = "".to_string();
        rbuf = "".to_string();
      },
      _ => {
        state = 0;
        lbuf = "".to_string();
        rbuf = "".to_string();
      },
    }
  }

  sum
}
