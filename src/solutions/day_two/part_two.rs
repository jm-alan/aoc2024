use super::{gen_reports, is_safe};

pub fn solution() -> i16 {
  gen_reports().into_iter().fold(0, |sum, report| {
    return is_safe_dampened(&report) as i16 + sum;
  })
}

fn is_safe_dampened(report: &[i8]) -> bool {
  if is_safe(report) {
    return true;
  }

  let len = report.len();
  let mut dampened = vec![0; len - 1];

  for skip in 0..len {
    for idx in 0..skip {
      dampened[idx] = report[idx];
    }
    for idx in (skip + 1)..len {
      dampened[idx - 1] = report[idx]
    }
    if is_safe(&dampened) {
      return true;
    }
  }

  false
}
