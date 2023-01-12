struct Solution {}

impl Solution {
  pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
      for j in 0..grid[0].len() {
        if i == 0 && j == 0 {
          buf[i][j] = grid[0][0];
          continue;
        }
        let mut left: i32 = 1000;
        let mut top: i32 = 1000;
        if i > 0 {
          top = buf[i - 1][j];
        }
        if j > 0 {
          left = buf[i][j - 1];
        }
        let mut v: i32 = left;
        if v > top {
          v = top
        }
        buf[i][j] = grid[i][j] + v;
      }
    }
    println!("{:?}", buf);
    buf[grid.len() - 1][grid[0].len() - 1]
  }
}
