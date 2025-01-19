impl Solution {
  pub fn count_texts(pressed_keys: String) -> i32 {
    let bb = pressed_keys.as_bytes().to_vec();

    let mut dp: Vec<i64> = vec![0; bb.len() + 1];
    dp[bb.len()] = 1;
    let N: i64 = 1000000007;

    let mut idx1: i32 = bb.len() as i32 - 1;
    while idx1 >= 0 {
      let mut sum: i64 = 0;
      let idx: usize = idx1 as usize;
      let ss = match bb[idx] {
        b'9' => 4,
        b'7' => 4,
        _ => 3,
      };
      let mut start = idx;
      while start < bb.len() && bb[idx] == bb[start] &&  start - idx < ss {
        sum += dp[start + 1];
        sum %= N;
        start += 1;
      }
      dp[idx] = sum;
      idx1 -= 1;
    }
    dp[0] as i32
  }
}
