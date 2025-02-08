struct Solution {}

impl Solution {
  pub fn unique_paths_with_obstacles(og: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; og[0].len()]; og.len()];

    (0..og.len()).for_each(|row| {
      (0..og[0].len()).for_each(|col| {
        if og[row][col] == 1 {
          return;
        }
        dp[row][col] = match (row, col) {
          (0, 0) => 1,
          (0, _) => dp[0][col - 1],
          (_, 0) => dp[row - 1][0],
          (_, _) => dp[row - 1][col] + dp[row][col - 1],
        };
      });
    });
    dp[dp.len() - 1][dp[0].len() - 1]
  }
}
