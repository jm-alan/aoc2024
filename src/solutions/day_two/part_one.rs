use super::{is_safe, REPORTS};

pub fn solution() -> i64 {
  REPORTS
    .iter()
    .fold(0, |sum, report| sum + (is_safe(report) as i64))
}
