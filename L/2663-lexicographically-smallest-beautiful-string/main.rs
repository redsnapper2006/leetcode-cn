struct Solution {}

impl Solution {
  pub fn smallest_beautiful_string(s: String, k: i32) -> String {
    let mut bb: Vec<u8> = s.as_bytes().to_vec();
    let mut idx: usize = bb.len() - 1;
    bb[idx] = bb[idx] + 1;
    while idx < bb.len() {
      if bb[idx] == (b'a' + k as u8) {
        if idx == 0 {
          return "".to_string();
        }
        bb[idx] = b'a';
        idx -= 1;
        bb[idx] += 1;
      } else if (idx >= 1 && bb[idx] == bb[idx - 1]) || (idx > 1 && bb[idx] == bb[idx - 2]) {
        bb[idx] += 1;
      } else {
        idx += 1;
      }
    }
    String::from_utf8(bb).unwrap()
  }
}
