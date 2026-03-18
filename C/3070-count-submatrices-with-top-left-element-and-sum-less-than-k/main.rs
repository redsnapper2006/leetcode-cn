impl Solution {
  pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; grid[0].len() + 1]; 2];

    let mut ans: i32 = 0;
    (0..grid.len()).for_each(|row| {
      (1..grid[0].len() + 1).for_each(|col| {
        buf[row % 2][col] = buf[row % 2][col - 1] + buf[(row + 1) % 2][col]
          - buf[(row + 1) % 2][col - 1]
          + grid[row][col - 1];
        ans += if buf[row % 2][col] <= k { 1 } else { 0 };
      });
    });
    ans
  }

  pub fn count_submatrices2(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];

    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        if row == 0 && col == 0 {
          buf[row][col] = grid[row][col];
        } else if row == 0 {
          buf[row][col] = buf[row][col - 1] + grid[row][col];
        } else if col == 0 {
          buf[row][col] = buf[row - 1][col] + grid[row][col];
        } else {
          buf[row][col] =
            buf[row - 1][col] + buf[row][col - 1] - buf[row - 1][col - 1] + grid[row][col];
        }
      });
    });

    buf.iter().flatten().filter(|x| **x <= k).count() as i32
  }
}
