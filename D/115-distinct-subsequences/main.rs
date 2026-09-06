impl Solution {
  pub fn num_distinct(s: String, t: String) -> i32 {
    let bs = s.as_bytes();
    let bt = t.as_bytes();
    if bs.len() < bt.len() {
      return 0;
    }

    let mut dp: Vec<Vec<i32>> = vec![vec![0; bt.len() + 1]; bs.len() + 1];
    for s in 0..bs.len() + 1 {
      dp[s][bt.len()] = 1;
    }
    for s in (0..bs.len()).rev() {
      for t in (0..bt.len()).rev() {
        dp[s][t] = dp[s + 1][t] + if bs[s] == bt[t] { dp[s + 1][t + 1] } else { 0 };
      }
    }
    dp[0][0]
  }

  pub fn num_distinct2(s: String, t: String) -> i32 {
    let bs = s.as_bytes();
    let bt = t.as_bytes();
    if bs.len() < bt.len() {
      return 0;
    }

    let mut cache: Vec<Vec<i32>> = vec![vec![-1; bt.len() + 1]; bs.len() + 1];

    fn recur(bs: &[u8], bt: &[u8], idx_s: usize, idx_t: usize, cache: &mut [Vec<i32>]) -> i32 {
      if cache[idx_s][idx_t] != -1 {
        return cache[idx_s][idx_t];
      }
      if idx_t == bt.len() {
        cache[idx_s][idx_t] = 1;
        return cache[idx_s][idx_t];
      }

      let mut ans: i32 = 0;
      for s in idx_s..=(bs.len() - (bt.len() - idx_t)) {
        if bs[s] == bt[idx_t] {
          ans += recur(bs, bt, s + 1, idx_t + 1, cache);
        }
      }
      cache[idx_s][idx_t] = ans;
      ans
    }
    recur(&bs, &bt, 0, 0, &mut cache)
  }
}
