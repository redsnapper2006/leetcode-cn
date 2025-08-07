impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
      fn dfs(grid:&mut Vec<Vec<i32>>,r:usize, c:usize) -> i32  {
        let cords : Vec<(i32, i32)> = vec![(-1,0),(1,0),(0,-1),(0,1)];

        let mut sum = grid[r][c];
        grid[r][c] = 0;
        for cord in cords.iter() {
          let nr = r as i32 + cord.0;
          let nc = c as i32 + cord.1;
          if nr < 0 || nr >= grid.len( ) || nc < 0 || nc >=grid[0].len() || grid[nr][nc] == 0  {
            continue;
          }
          sum += dfs(grid, nr ,nc);
        }
        sum
      }

      let mut ans : i32 = 0;
      for r in 0..grid.len() {
        for c in 0..grid[0].len() {
          if grid[r][c] == 0 {
            continue;
          }
          if dfs(&mut grid, r, c) % k == 0 {
            ans += 1;
          }
        }
      }
      ans
    }
}
