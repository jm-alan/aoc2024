use super::{gen_reports, is_safe};

pub fn solution() -> i16 {
  gen_reports()
    .into_iter()
    .fold(0, |sum, report| sum + (is_safe(&report) as i16))
}
