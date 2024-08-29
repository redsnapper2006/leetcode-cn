struct Solution {}

impl Solution {
  pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
    let mut row: usize = 0;
    while row < grid.len() {
      let mut col: usize = 0;
      while col < grid[0].len() {
        if row < grid.len() - 1 && grid[row][col] != grid[row + 1][col]
          || col < grid[0].len() - 1 && grid[row][col] == grid[row][col + 1]
        {
          return false;
        }
        col += 1;
      }
      row += 1;
    }
    true
  }
}
