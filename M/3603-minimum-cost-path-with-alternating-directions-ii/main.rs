impl Solution {
  pub fn min_cost(m: i32, n: i32, wait_cost: Vec<Vec<i32>>) -> i64 {
    let m = m as usize;
    let n = n as usize;
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; m];

    dp[0][0] = 1;
    for i in 0..m {
      for j in 0..n {
        if i == 0 && j == 0 {
          dp[i][j] = 1;
          continue;
        }
        dp[i][j] = (if i == 0 { i64::MAX } else { dp[i - 1][j] }).min(if j == 0 {
          i64::MAX
        } else {
          dp[i][j - 1]
        }) + (i as i64 + 1) * (j as i64 + 1)
          + wait_cost[i][j] as i64;
      }
    }

    dp[m - 1][n - 1] - wait_cost[m - 1][n - 1] as i64
  }
}
