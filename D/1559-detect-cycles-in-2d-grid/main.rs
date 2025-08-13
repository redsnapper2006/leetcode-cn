impl Solution {
  pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
    let mut grid = grid;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    fn dfs(grid: &Vec<Vec<char>>, r: usize, c: usize, dp: &mut Vec<Vec<i32>>, cnt: i32) -> bool {
      let cords: Vec<Vec<i32>> = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
      for cord in cords.iter() {
        let nr: i32 = r as i32 + cord[0];
        let nc: i32 = c as i32 + cord[1];
        if nr < 0
          || nr >= grid.len() as i32
          || nc < 0
          || nc >= grid[0].len() as i32
          || grid[r][c] != grid[nr as usize][nc as usize]
          || dp[nr as usize][nc as usize] != 0
        {
          continue;
        }
        dp[nr as usize][nc as usize] = cnt + 1;

        for i in 0..cords.len() {
          let nr2: i32 = nr + cords[i][0];
          let nc2: i32 = nc + cords[i][1];
          if nr2 < 0
            || nr2 >= grid.len() as i32
            || nc2 < 0
            || nc2 >= grid[0].len() as i32
            || grid[nr as usize][nc as usize] != grid[nr2 as usize][nc2 as usize]
          {
            continue;
          }
          if dp[nr as usize][nc as usize] > 0
            && dp[nr2 as usize][nc2 as usize] > 0
            && (dp[nr as usize][nc as usize] - dp[nr2 as usize][nc2 as usize]).abs() >= 3
          {
            return true;
          }
        }
        let t = dfs(grid, nr as usize, nc as usize, dp, cnt + 1);
        if t {
          return true;
        }
      }
      false
    }

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if grid[r][c] >= 'a' && grid[r][c] <= 'z' {
          dp[r][c] = 1;
          let t = dfs(&grid, r, c, &mut dp, 1);
          if t {
            return true;
          }
        }
      }
    }
    false
  }
}
