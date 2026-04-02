impl Solution {
  pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<(i32, i32, i32)>> =
      vec![vec![(i32::MIN, i32::MIN, i32::MIN); coins[0].len() + 1]; coins.len() + 1];
    dp[0][1].0 = 0;
    dp[1][0].0 = 0;

    for r in 0..coins.len() {
      for c in 0..coins[0].len() {
        dp[r + 1][c + 1].0 = dp[r][c + 1].0.max(dp[r + 1][c].0) + coins[r][c];

        let prev = dp[r][c + 1].1.max(dp[r + 1][c].1);
        if (prev != i32::MIN) {
          dp[r + 1][c + 1].1 = prev + coins[r][c];
        }

        if coins[r][c] < 0 {
          dp[r + 1][c + 1].1 = dp[r + 1][c + 1].1.max(dp[r][c + 1].0.max(dp[r + 1][c].0));
        }

        let prev2 = dp[r][c + 1].2.max(dp[r + 1][c].2);
        if (prev2 != i32::MIN) {
          dp[r + 1][c + 1].2 = prev2 + coins[r][c];
        }

        if coins[r][c] < 0 {
          dp[r + 1][c + 1].2 = dp[r + 1][c + 1].2.max(dp[r][c + 1].1.max(dp[r + 1][c].1));
        }
      }
    }
    let (v1, v2, v3) = dp[coins.len()][coins[0].len()];
    v1.max(v2).max(v3)
  }
}
