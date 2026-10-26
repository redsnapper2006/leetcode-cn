impl Solution {
  pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    const CORDS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let row: i64 = grid.len() as i64;
    let col: i64 = grid[0].len() as i64;
    fn dfs(grid: &mut [Vec<i32>], row: i64, col: i64, r: i64, c: i64) -> i64 {
      let mut sum = grid[r as usize][c as usize] as i64;
      grid[r as usize][c as usize] = 0;
      for (cr, cc) in CORDS {
        let (nr, nc) = (r + cr, c + cc);
        if nr >= 0 && nr < row && nc >= 0 && nc < col && grid[nr as usize][nc as usize] != 0 {
          sum += dfs(grid, row, col, nr, nc);
        }
      }
      sum
    }

    let mut ans: i32 = 0;
    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if grid[r][c] != 0 && dfs(&mut grid, row, col, r as i64, c as i64) % k as i64 == 0 {
          ans += 1;
        }
      }
    }
    ans
  }
}
