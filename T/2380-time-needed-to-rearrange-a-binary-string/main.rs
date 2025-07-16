impl Solution {
  pub fn seconds_to_remove_occurrences(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();

    let mut cnt: i32 = 0;
    let mut ans: i32 = 0;
    bb.iter().for_each(|&b| {
      if b == b'0' {
        cnt += 1;
      } else if cnt > 0 {
        ans = (ans + 1).max(cnt);
      }
    });
    ans
  }
}
