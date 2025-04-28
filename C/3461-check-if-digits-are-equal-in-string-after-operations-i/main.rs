impl Solution {
  pub fn has_same_digits(s: String) -> bool {
    let mut bb = s
      .as_bytes()
      .iter()
      .map(|&b| (b - b'0') as i32)
      .collect::<Vec<i32>>();
    while bb.len() > 2 {
      bb = (1..bb.len())
        .map(|idx| (bb[idx] + bb[idx - 1]) % 10)
        .collect::<Vec<i32>>();
    }
    bb[0] == bb[1]
  }
}
