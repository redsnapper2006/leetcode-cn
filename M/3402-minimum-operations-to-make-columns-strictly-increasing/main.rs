impl Solution {
  pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    (0..grid[0].len()).fold(0, |sum, col| {
      sum
        + (0..grid.len())
          .fold((0, grid[0][col] - 1), |(rsum, prev), row| {
            (
              rsum + ((prev + 1).max(grid[row][col]) - grid[row][col]),
              (prev + 1).max(grid[row][col]),
            )
          })
          .0
    })
  }
}
