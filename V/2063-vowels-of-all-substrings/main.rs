impl Solution {
  pub fn count_vowels(word: String) -> i64 {
    let bb = word.as_bytes().to_vec();
    let n = bb.len();
    bb.iter().enumerate().fold(0i64, |ans, (i, b)| match b {
      b'a' | b'e' | b'i' | b'o' | b'u' => ans + (n - i) as i64 * (i + 1) as i64,
      _ => ans,
    })
  }
}
