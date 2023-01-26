struct Solution {}

impl Solution {
  pub fn greatest_letter(s: String) -> String {
    let mut upper: [i32] = [0; 26];
    let mut lower: [i32] = [0; 26];
    for b in s.as_bytes() {
      if b <= 'Z' as u8 && b >= 'A' as u8 {
        upper[b - 'A'] = 1;
      } else {
        lower[b - 'a'] = 1;
      }
    }
    for i in (0..26).rev() {
      if upper[i] == 1 && lower[i] == 1 {
        return ('A' + i).to_string();
      }
    }
    ""
  }
}
