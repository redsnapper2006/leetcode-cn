struct Solution {}

impl Solution {
  pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    (0..grid.len()).for_each(|r| {
      (0..grid[0].len()).for_each(|c| {
        let mut up: i32 = 0;
        let mut left: i32 = 0;
        if r > 0 {
          up = buf[r - 1][c];
        }
        if c > 0 {
          left = buf[r][c - 1];
        }
        let mut inter: i32 = up;
        if inter < left {
          inter = left;
        }
        buf[r][c] = inter + grid[r][c];
      });
    });

    buf[buf.len() - 1][buf[0].len() - 1]
  }
}
