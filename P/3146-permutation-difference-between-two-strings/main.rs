impl Solution {
  pub fn find_permutation_difference(s: String, t: String) -> i32 {
    let mut buf: Vec<i32> = vec![0; 26];
    s.as_bytes().iter().enumerate().for_each(|(idx, &b)| {
      let offset = (b - b'a') as usize;
      buf[offset] = idx as i32;
    });
    let mut ans: i32 = 0;
    t.as_bytes().iter().enumerate().for_each(|(idx, &b)| {
      let offset = (b - b'a') as usize;
      ans += (buf[offset] - idx as i32).abs();
    });
    ans
  }
}
