struct Solution {}

impl Solution {
  pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<[i64; 2]>> = vec![vec![[0; 2]; grid[0].len()]; grid.len()];

    for i in 0..grid.len() {
      for j in 0..grid[0].len() {
        let mut max: i64;
        let mut min: i64;
        if i == 0 && j == 0 {
          max = grid[i][j] as i64;
          min = grid[i][j] as i64;
        } else if i == 0 || j == 0 {
          let mut pmax: i64;
          let mut pmin: i64;
          if i == 0 {
            pmax = buf[i][j - 1][1];
            pmin = buf[i][j - 1][0];
          } else {
            pmax = buf[i - 1][j][1];
            pmin = buf[i - 1][j][0];
          }

          pmax *= grid[i][j] as i64;
          pmin *= grid[i][j] as i64;
          if pmax > pmin {
            max = pmax;
            min = pmin;
          } else {
            max = pmin;
            min = pmax;
          }
        } else {
          let mut p1max = buf[i][j - 1][1];
          let mut p1min = buf[i][j - 1][0];
          let mut p2max = buf[i - 1][j][1];
          let mut p2min = buf[i - 1][j][0];
          p1max *= grid[i][j] as i64;
          p1min *= grid[i][j] as i64;
          p2max *= grid[i][j] as i64;
          p2min *= grid[i][j] as i64;

          max = p1max;
          if max < p1min {
            max = p1min;
          }
          if max < p2max {
            max = p2max;
          }
          if max < p2min {
            max = p2min;
          }
          min = p1max;
          if min > p1min {
            min = p1min;
          }
          if min > p2max {
            min = p2max;
          }
          if min > p2min {
            min = p2min;
          }
        }
        buf[i][j] = [min, max];
      }
    }

    if buf[grid.len() - 1][grid[0].len() - 1][1] < 0 {
      return -1;
    }
    (buf[grid.len() - 1][grid[0].len() - 1][1] % 1000000007) as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_product_path(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]])
  );
  println!(
    "{}",
    Solution::max_product_path(vec![
      vec![1, -1, 2, 1, -1, 0, 0, 4, 3, 2, 0, -2, -2],
      vec![-2, 3, 3, -1, -1, 0, 0, -2, 4, -3, 3, 0, 0],
      vec![-4, -1, -1, -2, 2, -1, -2, -2, 0, 3, -1, -4, 1],
      vec![-3, 4, -3, 0, -3, 1, -3, 1, 4, 4, -4, -4, -2],
      vec![3, -3, 1, 0, -1, -4, -4, -4, 3, 2, 2, 3, 3],
      vec![2, -1, -1, -4, -3, -3, 4, 2, 3, 4, 4, -4, 0],
      vec![4, -1, 2, -3, -1, -1, -3, -4, 4, 4, 4, -3, -1],
      vec![-3, -4, 4, -2, -1, 2, 3, -1, 2, 3, 4, 4, -4],
      vec![-3, -1, -2, 1, 1, -1, -3, -4, -3, 1, -3, 3, -4],
      vec![2, 4, 4, 4, -3, -3, 1, -1, 3, 4, -1, 1, 4],
      vec![2, -2, 0, 4, -1, 0, -2, 4, -4, 0, 0, 2, -3],
      vec![1, 1, -3, 0, -4, -4, -4, -4, 0, -1, -4, -1, 0],
      vec![3, -1, -3, -3, -3, -2, -1, 4, -1, -2, 4, 2, 3]
    ])
  );
}
