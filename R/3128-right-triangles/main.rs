struct Solution {}

impl Solution {
  pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
    let mut row: Vec<i64> = vec![0; grid.len()];
    let mut col: Vec<i64> = vec![0; grid[0].len()];
    (0..grid.len()).for_each(|r| {
      (0..grid[0].len()).for_each(|c| {
        row[r] += grid[r][c] as i64;
        col[c] += grid[r][c] as i64;
      });
    });

    let mut ans: i64 = 0;
    (0..grid.len()).for_each(|r| {
      (0..grid[0].len()).for_each(|c| {
        if grid[r][c] == 0 {
          return;
        }
        ans += (row[r] - 1) * (col[c] - 1);
      });
    });
    ans
  }
}
