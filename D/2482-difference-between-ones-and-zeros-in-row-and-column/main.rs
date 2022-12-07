struct Solution {}

impl Solution {
  pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows: Vec<i32> = grid
      .iter()
      .map(|row| row.iter().map(|x| 2 * x - 1).sum())
      .collect();
    let cols: Vec<i32> = (0..grid[0].len())
      .map(|cIdx| grid.iter().map(|row| 2 * row[cIdx] - 1).sum())
      .collect();

    let mut ret: Vec<Vec<i32>> = Vec::new();
    for i in 0..grid.len() {
      let mut rvec: Vec<i32> = Vec::new();
      for j in 0..grid[0].len() {
        rvec.push(rows[i] + cols[j]);
      }
      ret.push(rvec);
    }
    ret
  }
}
