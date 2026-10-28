impl Solution {
  pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<bool>> = vec![vec![false; 1024]; grid[0].len()];

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if r == 0 && c == 0 {
          dp[0][grid[r][c] as usize] = true;
        } else {
          let mut buf: Vec<bool> = vec![false; 1024];
          for i in 0..1024 {
            if dp[c][i] || c > 0 && dp[c - 1][i] {
              buf[i ^ grid[r][c] as usize] = true;
            }
          }
          dp[c] = buf;
        }
      }
    }
    for i in 0..1024 {
      if dp[grid[0].len() - 1][i] {
        return i as _;
      }
    }
    -1
  }
}
