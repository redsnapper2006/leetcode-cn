struct Solution {}

impl Solution {
  pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![1 << 31 - 1; grid[0].len()]; grid.len()];
    buf[0] = vec![0; grid[0].len()];

    (0..grid.len() - 1).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        let key = grid[row][col];

        (0..grid[0].len()).for_each(|off| {
          buf[row + 1][off] =
            buf[row + 1][off].min(buf[row][col] + key + move_cost[key as usize][off]);
        });
      });
    });

    let last_row: usize = buf.len() - 1;
    (0..grid[0].len()).for_each(|col| {
      buf[last_row][col] += grid[last_row][col];
    });

    *buf[buf.len() - 1].iter().min().unwrap()
  }
}
