struct Solution {}

impl Solution {
  pub fn repeated_character(s: String) -> char {
    let mut buf: [i32; 26] = [0; 26];
    for c in s.chars() {
      let offset = c - 'a' as u8;
      buf[offset] += 1;
      if buf[offset] == 2 {
        return c;
      }
    }
    'a'
  }
}
