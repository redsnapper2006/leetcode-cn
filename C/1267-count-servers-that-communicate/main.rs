impl Solution {
  pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut row: Vec<i32> = vec![0; grid.len()];
    let mut col: Vec<i32> = vec![0; grid[0].len()];
    (0..grid.len()).for_each(|i| {
      let mut sum: i32 = 0;
      (0..grid[0].len()).for_each(|j| {
        sum += grid[i][j];
      });
      row[i] = sum;
    });
    (0..grid[0].len()).for_each(|j| {
      let mut sum: i32 = 0;
      (0..grid.len()).for_each(|i| {
        sum += grid[i][j];
      });
      col[j] = sum;
    });

    let mut total: i32 = 0;
    (0..grid.len()).for_each(|i| {
      (0..grid[0].len()).for_each(|j| {
        if grid[i][j] == 1 && (row[i] > 1 || col[j] > 1) {
          total += 1;
        }
      });
    });
    total
  }
}
