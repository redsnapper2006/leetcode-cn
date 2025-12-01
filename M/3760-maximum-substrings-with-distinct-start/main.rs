impl Solution {
  pub fn max_distinct(s: String) -> i32 {
    s.as_bytes()
      .iter()
      .fold(vec![0; 26], |mut buf, b| {
        buf[(b - b'a') as usize] = 1;
        buf
      })
      .iter()
      .sum()
  }
}
