impl Solution {
  pub fn min_deletion(s: String, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];

    s.as_bytes().iter().for_each(|b| {
      buf[(b - b'a') as usize] += 1;
    });
    buf.sort_unstable();

    let mut ans: i32 = 0;
    (0..(26 - k)).for_each(|idx| {
      ans += buf[idx as usize];
    });
    ans
  }
}
