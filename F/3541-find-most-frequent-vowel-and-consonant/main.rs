impl Solution {
  pub fn max_freq_sum(s: String) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    s.as_bytes().iter().for_each(|&b| {
      buf[(b - b'a') as usize] += 1;
    });

    let mut vowel: i32 = 0;
    let mut non_vowel: i32 = 0;
    (0..26).for_each(|idx| {
      if idx == 0 || idx == 4 || idx == 8 || idx == 14 || idx == 20 {
        vowel = vowel.max(buf[idx]);
      } else {
        non_vowel = non_vowel.max(buf[idx]);
      }
    });
    vowel + non_vowel
  }
}
