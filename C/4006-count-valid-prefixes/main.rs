impl Solution {
  pub fn count_valid_prefixes(s: String) -> i32 {
    s.as_bytes()
      .iter()
      .fold((0i32, 0i32, 0i32), |(ans, zero, one), b| {
        let d = (b - b'0') as i32;
        (
          ans
            + if (zero + 1 - d - one - d).abs() <= 1 {
              1
            } else {
              0
            },
          zero + 1 - d,
          one + d,
        )
      })
      .0
  }
}
