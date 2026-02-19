impl Solution {
  pub fn count_binary_substrings(s: String) -> i32 {
    s.as_bytes()
      .iter()
      .fold((0, 0, b' ', 0), |(ans, prev_cnt, base, base_cnt), &b| {
        if base == b {
          (ans + if prev_cnt > 0 { 1 } else { 0 }, prev_cnt - 1, b, base_cnt + 1)
        } else {
          (ans + if base_cnt > 0 { 1 } else { 0 }, base_cnt - 1, b, 1)
        }
      })
      .0
  }
}
