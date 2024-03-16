struct Solution {}



impl Solution {
  pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    (1..grid[0].len()).for_each(|col| {
      (0..grid.len()).for_each(|row| {
        let mut max: i32 = if grid[row][col - 1] < grid[row][col] && dp[row][col - 1] == col as i32 - 1 {
          dp[row][col-1] + 1
        } else {
          0
        };
        if row > 0 && grid[row - 1][col - 1] < grid[row][col] && dp[row - 1][col - 1] == col as i32 - 1 {
          max = max.max(dp[row - 1][col - 1] + 1);
        }
        if row < grid.len() - 1
          && grid[row + 1][col - 1] < grid[row][col]
          && dp[row + 1][col - 1] == col as i32 - 1
        {
          max = max.max(dp[row + 1][col - 1] + 1);
        }
        dp[row][col] = max;
      });
    });

    let mut max: i32 = 0;
    dp.iter().for_each(|row| {
      if max < *row.iter().max().unwrap() {
        max = *row.iter().max().unwrap();
      }
    });
    max
  }
}
