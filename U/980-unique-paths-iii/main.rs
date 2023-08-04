impl Solution {
  pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    while row < grid.len() as i32 {
      col = 0;
      while col < grid[0].len() as i32 {
        if grid[row as usize][col as usize] == 1 {
          break;
        }
        col += 1;
      }
      if col < grid[0].len() as i32 && grid[row as usize][col as usize] == 1 {
        break;
      }
      row += 1;
    }

    fn dfs(row: i32, col: i32, grid: &mut Vec<Vec<i32>>, res: &mut i32) {
      let v = grid[row as usize][col as usize];
      // let mut is_recover: bool = false;
      if v == 0 {
        grid[row as usize][col as usize] = 3;
        // is_recover = true;
      }
      if v == 2 {
        let mut is_ok: bool = true;
        for r in (0..grid.len()) {
          for c in (0..grid[0].len()) {
            if grid[r][c] == 0 {
              is_ok = false;
              break;
            }
          }
          if !is_ok {
            break;
          }
        }
        if is_ok {
          *res += 1;
        }
        return;
      }
      for cord in vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]] {
        let new_row = row + cord[0];
        let new_col = col + cord[1];
        println!("{:?} {} {}", grid, new_row, new_col);
        if new_row >= 0
          && new_row < grid.len() as i32
          && new_col >= 0
          && new_col < grid[0].len() as i32
          && (grid[new_row as usize][new_col as usize] == 0
            || grid[new_row as usize][new_col as usize] == 2)
        {
          dfs(new_row, new_col, grid, res);
        }
      }
      grid[row as usize][col as usize] = 0;
    }

    let mut res: i32 = 0;
    dfs(row, col, &mut grid.clone(), &mut res);
    res
  }
}
