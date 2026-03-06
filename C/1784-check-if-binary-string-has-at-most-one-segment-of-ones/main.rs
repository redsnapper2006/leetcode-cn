impl Solution {
  pub fn check_ones_segment(s: String) -> bool {
    let b = s.as_bytes();
    for i in 1..b.len() {
      if b[i - 1] == b'0' && b[i] == b'1' {
        return false;
      }
    }
    true
  }

  pub fn check_ones_segment2(s: String) -> bool {
    !s.as_bytes().windows(2).any(|w| w == [b'0', b'1'])
  }
}
