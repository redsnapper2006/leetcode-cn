impl Solution {
  pub fn max_depth(s: String) -> i32 {
    s.bytes()
      .fold((0, 0), |(ans, depth), b| match b {
        b'(' => (ans, depth + 1),
        b')' => (ans.max(depth), depth - 1),
        _ => (ans, depth),
      })
      .0
  }
}
