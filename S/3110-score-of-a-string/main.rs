impl Solution {
  pub fn score_of_string(s: String) -> i32 {
    let bb = s.as_bytes();
    (1..bb.len()).fold(0, |sum, idx| {
      (bb[idx] as i32 - bb[idx - 1] as i32).abs() + sum
    })
  }
}
