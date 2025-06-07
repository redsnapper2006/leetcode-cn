impl Solution {
  pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32, sum: &mut i32) {
      let coord: Vec<Vec<i32>> = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];

      coord.iter().for_each(|cc| {
        let nr = r + cc[0];
        let nc = c + cc[1];
        if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[0].len() as i32 {
          return;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        if grid[nr][nc] == 0 {
          return;
        }
        *sum += grid[nr][nc];
        grid[nr][nc] = 0;
        dfs(grid, nr as i32, nc as i32, sum);
      });
    }

    let mut grid = grid;
    let mut ans: i32 = 0;
    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        if grid[row][col] == 0 {
          return;
        }
        let mut sum: i32 = grid[row][col];
        grid[row][col] = 0;
        dfs(&mut grid, row as i32, col as i32, &mut sum);
        ans = ans.max(sum);
      });
    });
    ans
  }
}
