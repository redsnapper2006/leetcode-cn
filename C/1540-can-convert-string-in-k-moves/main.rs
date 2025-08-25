impl Solution {
  pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
    let bs: Vec<u8> = s.as_bytes().to_vec();
    let bt: Vec<u8> = t.as_bytes().to_vec();
    if bs.len() != bt.len() {
      return false;
    }
    let mut buf: Vec<i32> = vec![0; bs.len()];
    for i in 0..bs.len() {
      let mut diff: i32 = bt[i] as i32 - bs[i] as i32;
      if diff < 0 {
        diff += 26;
      }
      buf[i] = diff;
    }
    let mut dp: Vec<i32> = vec![0; 26];
    let mut ans: i32 = 0;
    for i in 0..buf.len() {
      if buf[i] == 0 {
        continue;
      }
      dp[buf[i] as usize] += 1;
    }
    for i in 1..26 {
      if dp[i] == 0 {
        continue;
      }
      ans = ans.max((dp[i] - 1) * 26 + i as i32);
    }
    // println!("{:?} {}", dp, ans);
    ans <= k
  }
}
