impl Solution {
  pub fn is_valid(word: String) -> bool {
    if word.len() < 3 {
      return false;
    }

    let bb = word.as_bytes().to_vec();
    let mut is_vowel: bool = false;
    let mut is_p: bool = false;
    for i in 0..bb.len() {
      let b = bb[i];
      if (b >= b'a' && b <= b'z') || (b >= b'A' && b <= b'Z') || (b >= b'0' && b <= b'9') {
        if !(b >= b'0' && b <= b'9') {
          if b == b'a'
            || b == b'e'
            || b == b'i'
            || b == b'o'
            || b == b'u'
            || b == b'A'
            || b == b'E'
            || b == b'I'
            || b == b'O'
            || b == b'U'
          {
            is_vowel = true;
          } else {
            is_p = true;
          }
        }
      } else {
        return false;
      }
    }
    is_vowel && is_p
  }
}
