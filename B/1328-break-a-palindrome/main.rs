struct Solution {}

impl Solution {
  pub fn break_palindrome(palindrome: String) -> String {
    let mut bb = palindrome.as_bytes().to_vec();
    let size = bb.len();
    if bb.len() == 1 {
      return "".to_string();
    }

    let mut offset = size - 1;
    let mut replace: u8 = b'b';

    let mut idx: usize = 0;
    while idx * 2 + 1 < size {
      if bb[idx] != b'a' {
        offset = idx;
        replace = b'a';
        break;
      }
      idx += 1;
    }
    bb[offset] = replace;

    String::from_utf8(bb).unwrap()
  }
}
