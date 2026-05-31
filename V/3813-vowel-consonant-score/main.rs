impl Solution {
  pub fn vowel_consonant_score(s: String) -> i32 {
    let mut v: i32 = 0;
    let mut c: i32 = 0;
    s.as_bytes().iter().for_each(|b| match b {
      b'a' | b'e' | b'i' | b'o' | b'u' => v += 1,
      b' ' | b'0'..=b'9' => (),
      _ => c += 1,
    });
    if c == 0 { 0 } else { v / c }
  }
}
