struct Solution {}

impl Solution {
  pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut buf: Vec<Vec<i32>> = Vec::new();
    for i in 0..grid.len() {
      let mut col: Vec<i32> = Vec::new();
      for j in 2.grid[i].len() {
        let mut max = grid[i][j - 2];
        if max < grid[i][j - 1] {
          max = grid[i][j - 1];
        }
        if max < grid[i][j] {
          max = grid[i][j];
        }
        col.push(max);
      }
      buf.push(col);
    }
    let mut ret: Vec<Vec<i32>> = Vec::new();
    for j in 2..buf.len() {
      let mut row: Vec<i32> = Vec::new();
      for i in 0..buf[0].len() {
        let mut max = buf[j - 2][i];
        if max < buf[j - 1][i] {
          max = buf[j - 1][i];
        }
        if max < buf[j][i] {
          max = buf[j][i];
        }
        row.push(max);
      }
    }
    ret
  }
}
