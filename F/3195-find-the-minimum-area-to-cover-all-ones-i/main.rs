impl Solution {
  pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
    let (mut minr, mut minc, mut maxr, mut maxc): (i32, i32, i32, i32) = (10000, 10000, -1, -1);
    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        if grid[row][col] == 0 {
          return;
        }
        minr = minr.min(row as i32);
        maxr = maxr.max(row as i32);
        minc = minc.min(col as i32);
        maxc = maxc.max(col as i32);
      });
    });
    (maxr - minr + 1) * (maxc - minc + 1)
  }
}
