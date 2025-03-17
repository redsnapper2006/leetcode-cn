struct Solution {}

impl Solution {
  pub fn min_swaps(s: String) -> i32 {
    s.as_bytes().iter().fold(0, |sum, &b| {
      sum + if b == b'[' || sum == 0 { 1 } else { -1 }
    }) / 2
  }
}
