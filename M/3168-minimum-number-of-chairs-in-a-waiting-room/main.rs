struct Solution {}

impl Solution {
  pub fn minimum_chairs(s: String) -> i32 {
    s.as_bytes()
      .iter()
      .fold((0, 0), |(max, cnt), &b| match b {
        b'E' => (max.max(cnt + 1), cnt + 1),
        _ => (max, cnt - 1),
      })
      .0
  }
}
