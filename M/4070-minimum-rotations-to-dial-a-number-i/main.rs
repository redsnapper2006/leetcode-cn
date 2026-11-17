impl Solution {
  pub fn min_rotations(s: String) -> i32 {
    s.as_bytes()
      .iter()
      .fold((0i32, 0i32), |(ans, prev), &b| {
        let b = (b - b'0') as i32;
        let diff = if b > prev { b - prev } else { prev - b };
        (ans + diff.min(10 - diff), b)
      })
      .0
  }
}
