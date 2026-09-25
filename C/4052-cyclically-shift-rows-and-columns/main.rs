impl Solution {
  pub fn cyclic_shift(
    n: i32, grid: Vec<Vec<i32>>, row_shift: Vec<i32>, col_shift: Vec<i32>,
  ) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; n];
    for r in 0..n {
      for c in 0..n {
        let rr = (r + col_shift[c] as usize) % n;
        ans[r][c] = grid[rr][(c + row_shift[rr] as usize) % n];
      }
    }
    ans
  }
}
