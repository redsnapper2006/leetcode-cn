impl Solution {
  pub fn reverse_degree(s: String) -> i32 {
    s.as_bytes()
      .to_vec()
      .iter()
      .enumerate()
      .fold(0, |sum, (idx, b)| {
        sum + (26 - (b - b'a') as i32) * (idx as i32 + 1)
      })
  }
}
