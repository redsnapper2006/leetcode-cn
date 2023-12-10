struct Solution {}

impl Solution {
  pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; n as usize + 1];
    dp[0] = 1;
    (0..n as usize).for_each(|i| {
      dp[i + 1] += dp[i];
      if i + 2 <= n as usize {
        dp[i + 2] += dp[i];
      }
    });
    dp[n as usize]
  }
}
