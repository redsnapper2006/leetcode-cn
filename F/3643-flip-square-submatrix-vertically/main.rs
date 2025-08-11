impl Solution {
  pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
    let mut grid = grid;

    let mut x = x as usize;
    let mut e = (grid.len() - 1).min(x + k - 1);
    let y = y as usize;
    let k = k as usize;
    while x < e {
      for j in y..grid[0].len().min(y + k) {
        let t = grid[x][j];
        grid[x][j] = grid[e][j];
        grid[e][j] = t;
      }

      x += 1;
      e -= 1;
    }

    grid
  }
}
