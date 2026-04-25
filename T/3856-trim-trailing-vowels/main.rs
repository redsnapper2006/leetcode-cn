impl Solution {
  pub fn trim_trailing_vowels(s: String) -> String {
    let mut bb = s.as_bytes().to_vec();
    let mut idx: i32 = bb.len() as i32 - 1;
    while idx >= 0 {
      let i = idx as usize;
      if !(bb[i] == b'a' || bb[i] == b'e' || bb[i] == b'i' || bb[i] == b'o' || bb[i] == b'u') {
        break;
      } else {
        idx -= 1;
      }
    }

    String::from_utf8(bb[0..(idx + 1) as usize].to_vec()).unwrap()
  }
}
