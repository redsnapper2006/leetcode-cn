impl Solution {
  pub fn minimum_pushes(word: String) -> i32 {
    let n = word.len() as i32;
    let m = (n + 7) / 8;
    m * (m + 1) * 4 - (m * 8 - n) * m
  }

  pub fn minimum_pushes2(word: String) -> i32 {
    let mut cnt = [0i32; 26];
    for &b in word.as_bytes() {
      cnt[(b - b'a') as usize] += 1;
    }
    cnt.sort_unstable_by(|a, b| b.cmp(a));
    cnt.into_iter().enumerate().fold(0, |ans, (idx, v)| ans + v * (idx as i32 / 8 + 1))
  }
}

struct Solution {}
