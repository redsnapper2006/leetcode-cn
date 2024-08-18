struct Solution {}

impl Solution {
  pub fn check_record(s: String) -> bool {
    let r = s
      .as_bytes()
      .to_vec()
      .iter()
      .fold((0, false, 0), |(a, l, c), v| match v {
        b'A' => (a + 1, l, 0),
        b'L' => (a, l || c >= 2, c + 1),
        _ => (a, l, 0),
      });
    r.0 < 2 && !r.1
  }
}
