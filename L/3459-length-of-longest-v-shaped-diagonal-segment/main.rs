impl Solution {
  pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<(i32, i32, i32, i32)>> =
      vec![vec![(0, 0, 0, 0); grid[0].len()]; grid.len()];

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if grid[r][c] == 1 {
          continue;
        }
        let nr = r as i32 - 1;
        let nc = c as i32 + 1;
        if nr >= 0
          && nr < grid.len() as i32
          && nc >= 0
          && nc < grid[0].len() as i32
          && grid[r][c] + grid[nr as usize][nc as usize] == 2
        {
          dp[r][c].3 = dp[nr as usize][nc as usize].3 + 1;
        }
        let nr = r as i32 - 1;
        let nc = c as i32 - 1;
        if nr >= 0
          && nr < grid.len() as i32
          && nc >= 0
          && nc < grid[0].len() as i32
          && grid[r][c] + grid[nr as usize][nc as usize] == 2
        {
          dp[r][c].2 = dp[nr as usize][nc as usize].2 + 1;
        }
      }
    }

    for r in (0..grid.len()).rev() {
      for c in 0..grid[0].len() {
        if grid[r][c] == 1 {
          continue;
        }
        let nr = r as i32 + 1;
        let nc = c as i32 + 1;
        if nr >= 0
          && nr < grid.len() as i32
          && nc >= 0
          && nc < grid[0].len() as i32
          && grid[r][c] + grid[nr as usize][nc as usize] == 2
        {
          dp[r][c].0 = dp[nr as usize][nc as usize].0 + 1;
        }
        let nr = r as i32 + 1;
        let nc = c as i32 - 1;
        if nr >= 0
          && nr < grid.len() as i32
          && nc >= 0
          && nc < grid[0].len() as i32
          && grid[r][c] + grid[nr as usize][nc as usize] == 2
        {
          dp[r][c].1 = dp[nr as usize][nc as usize].1 + 1;
        }
      }
    }

    let mut ans: i32 = -2;
    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if grid[r][c] != 1 {
          continue;
        }
        ans = ans.max(-1);
        for cord in vec![(-1, -1), (-1, 1), (1, -1), (1, 1)].iter() {
          let mut nr = r as i32 + cord.0;
          let mut nc = c as i32 + cord.1;
          let mut digit: i32 = 2;
          while nr >= 0
            && nr < grid.len() as i32
            && nc >= 0
            && nc < grid[0].len() as i32
            && grid[nr as usize][nc as usize] == digit
          {
            let cur = dp[nr as usize][nc as usize];
            match cord {
              (-1, -1) => {
                ans = ans.max(cur.0 + cur.3);
              }
              (-1, 1) => {
                ans = ans.max(cur.1 + cur.0);
              }
              (1, 1) => {
                ans = ans.max(cur.2 + cur.1);
              }
              (_, _) => {
                ans = ans.max(cur.3 + cur.2);
              }
            };
            nr += cord.0;
            nc += cord.1;
            digit = 2 - digit;
          }
        }
      }
    }

    ans + 2
  }
}
