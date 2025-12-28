impl Solution {
  pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    (0..grid.len())
      .fold((0, grid[0].len() as i32 - 1), |(ans, col), row| {
        let mut c = col as i32;
        while c >= 0 && grid[row][c as usize] < 0 {
          c -= 1;
        }
        (ans + grid[0].len() as i32 - 1 - c, c)
      })
      .0
  }
}
