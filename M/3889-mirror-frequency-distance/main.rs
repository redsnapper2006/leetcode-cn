impl Solution {
  pub fn mirror_frequency(s: String) -> i32 {
    let mut letters: Vec<i32> = vec![0; 26];
    let mut digits: Vec<i32> = vec![0; 10];
    s.as_bytes().iter().for_each(|&b| match b {
      b'a'..=b'z' => {
        letters[(b - b'a') as usize] += 1;
      }
      _ => {
        digits[(b - b'0') as usize] += 1;
      }
    });

    (0..13).fold(0, |sum, idx| sum + (letters[idx] - letters[25 - idx]).abs())
      + (0..5).fold(0, |sum, idx| sum + (digits[idx] - digits[9 - idx]).abs())
  }
}
