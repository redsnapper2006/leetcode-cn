impl Solution {
  pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut suffix: Vec<Vec<i64>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut suf: i64 = 1;
    for r in (0..grid.len()).rev() {
      for c in (0..grid[0].len()).rev() {
        suffix[r][c] = suf;
        suf = (suf * grid[r][c] as i64) % 12345;
      }
    }

    let mut ans: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut pre: i64 = 1;
    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        ans[r][c] = ((pre * suffix[r][c]) % 12345) as i32;
        pre = (pre * grid[r][c] as i64) % 12345;
      }
    }

    ans
  }
}
