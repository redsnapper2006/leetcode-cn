struct Solution {}

impl Solution {
  pub fn check_x_matrix(grid: Vec<Vec<i32>>) -> bool {
    (0..grid.len())
      .map(|r| {
        (0..grid[0].len())
          .map(|c| match (r == c || r + c == grid.len() - 1, grid[r][c]) {
            (true, 0) => false,
            (false, 0) => true,
            (false, _) => false,
            (_, _) => true,
          })
          .collect::<Vec<bool>>()
      })
      .flatten()
      .all(|x| x)
  }
}
