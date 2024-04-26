impl Solution {
  pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    (0..grid[0].len()).fold(Vec::new(), |mut rv, col| {
      rv.push((0..grid.len()).fold(0, |max, row| {
        let mut digits: i32 = 0;
        let mut v = grid[row][col];
        if v <= 0 {
          digits += 1;
        }

        while v != 0 {
          v /= 10;
          digits += 1;
        }

        max.max(digits)
      }));
      rv
    })
  }
}
