impl Solution {
  pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; k]; grid[0].len()]; grid.len()];

    dp[0][0][grid[0][0] as usize % k] = 1;
    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if r > 0 {
          for m in 0..k {
            let idx = (m + grid[r][c] as usize) % k;
            dp[r][c][idx] += dp[r - 1][c][m];
            dp[r][c][idx] %= 1000000007;
          }
        }
        if c > 0 {
          for m in 0..k {
            let idx = (m + grid[r][c] as usize) % k;
            dp[r][c][idx] += dp[r][c - 1][m];
            dp[r][c][idx] %= 1000000007;
          }
        }
      }
    }
    dp[grid.len() - 1][grid[0].len() - 1][0]
  }
}
