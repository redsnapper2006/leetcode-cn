impl Solution {
  pub fn is_possible_to_cut_path(grid: Vec<Vec<i32>>) -> bool {
    let mut grid = grid;

    fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) -> bool {
      if r == grid.len() - 1 && c == grid[0].len() - 1 {
        return true;
      }

      grid[r][c] = 0;
      r < grid.len() - 1 && grid[r + 1][c] == 1 && dfs(grid, r + 1, c)
        || c < grid[0].len() - 1 && grid[r][c + 1] == 1 && dfs(grid, r, c + 1)
    }

    !dfs(&mut grid, 0, 0) || !dfs(&mut grid, 0, 0)
  }
}
