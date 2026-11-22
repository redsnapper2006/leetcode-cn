impl Solution {
  pub fn check_string(s: String) -> bool {
    let bb = s.as_bytes();
    for i in 1..bb.len() {
      if bb[i] == b'a' && bb[i - 1] == b'b' {
        return false;
      }
    }
    true
  }
}
