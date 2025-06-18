impl Solution {
  pub fn minimum_pushes(word: String) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    word.as_bytes().iter().for_each(|&b| {
      buf[(b - b'a') as usize] += 1;
    });

    buf.sort_unstable();
    let mut ans: i32 = 0;
    (0..26).rev().for_each(|idx| {
      ans += buf[idx] * ((25 - idx as i32) / 8 + 1);
    });
    ans
  }
}
