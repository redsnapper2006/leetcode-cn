impl Solution {
  pub fn num_sub(s: String) -> i32 {
    s.as_bytes()
      .iter()
      .fold((0, 0), |(ans, cnt), &b| {
        if b == b'1' {
          ((ans + cnt + 1) % 1000000007, cnt + 1)
        } else {
          (ans, 0)
        }
      })
      .0
  }
}
