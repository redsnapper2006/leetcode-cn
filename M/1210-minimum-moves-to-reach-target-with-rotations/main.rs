struct Solution {}

impl Solution {
  pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<[i32; 2]>> = vec![vec![[-1, -1]; grid[0].len()]; grid.len()];
    buf[0][0][0] = 0;
    if buf.len() > 1 && grid[1][0] == 0 && grid[1][1] == 0 {
      buf[0][0][1] = 1;
    }

    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        if grid[row][col] == 1 {
          return;
        }
        let mut h: i32 = buf[row][col][0];
        if col + 1 < grid[0].len() && grid[row][col + 1] == 0 {
          if col > 0 && buf[row][col - 1][0] != -1 && (h == -1 || h > buf[row][col - 1][0] + 1) {
            h = buf[row][col - 1][0] + 1;
          }
          if row > 0 && buf[row - 1][col][0] != -1 && (h == -1 || h > buf[row - 1][col][0] + 1) {
            h = buf[row - 1][col][0] + 1;
          }
        }

        let mut v: i32 = buf[row][col][1];
        if row + 1 < grid.len() && grid[row + 1][col] == 0 {
          if row > 0 && buf[row - 1][col][1] != -1 && (v == -1 || v > buf[row - 1][col][1] + 1) {
            v = buf[row - 1][col][1] + 1;
          }
          if col > 0 && buf[row][col - 1][1] != -1 && (v == -1 || v > buf[row][col - 1][1] + 1) {
            v = buf[row][col - 1][1] + 1;
          }
        }

        if row + 1 < grid.len()
          && col + 1 < grid[0].len()
          && grid[row + 1][col] == 0
          && grid[row][col + 1] == 0
          && grid[row + 1][col + 1] == 0
        {
          if v != -1 && (h == -1 || h > v + 1) {
            h = v + 1;
          }
          if h != -1 && (v == -1 || v > h + 1) {
            v = h + 1;
          }
        }
        buf[row][col][0] = h;
        buf[row][col][1] = v;
      });
    });

    buf[grid.len() - 1][grid[0].len() - 2][0]
  }
}
