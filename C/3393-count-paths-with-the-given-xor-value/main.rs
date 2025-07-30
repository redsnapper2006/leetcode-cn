use std::collections::HashMap;

impl Solution {
  pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let N: i64 = 1000000007;
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 16]; grid[0].len()]; grid.len()];
    dp[0][0][grid[0][0] as usize] = 1;
    for i in 0..grid.len() {
      for j in 0..grid[0].len() {
        if i == 0 && j == 0 {
          continue;
        }
        if i > 0 {
          for x in 0..16 {
            let n = x ^ grid[i][j] as usize;
            dp[i][j][n] = (dp[i][j][n] + dp[i - 1][j][x]) % N;
          }
        }
        if j > 0 {
          for x in 0..16 {
            let n = x ^ grid[i][j] as usize;
            dp[i][j][n] = (dp[i][j][n] + dp[i][j - 1][x]) % N;
          }
        }
      }
    }
    dp[grid.len() - 1][grid[0].len() - 1][k as usize] as _
  }
}
