impl Solution {
  pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let (row, col) = (grid.len(), grid[0].len());
    let mut ans: Vec<Vec<i32>> = vec![vec![0; col]; row];
    let k = k as usize % (row * col);
    for r in 0..row {
      for c in 0..col {
        let idx = (r * col + c + k) % (row * col);
        ans[idx / col][idx % col] = grid[r][c];
      }
    }
    ans
  }
}
