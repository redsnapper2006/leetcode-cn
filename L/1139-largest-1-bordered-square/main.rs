struct Solution {}

impl Solution {
  pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<[usize; 2]>> = Vec::new();

    (0..grid.len()).for_each(|r| {
      let mut col: Vec<[usize; 2]> = Vec::new();
      (0..grid[0].len()).for_each(|c| {
        if grid[r][c] == 0 {
          col.push([0, 0]);
          return;
        }
        let mut v: [usize; 2] = [0, 0];
        match (r, c) {
          (0, 0) => {
            v = [grid[r][c] as usize, grid[r][c] as usize];
          }
          (0, _) => {
            v = [col[c - 1][0] + 1, 1];
          }
          (_, 0) => {
            v = [1, buf[r - 1][c][1] + 1];
          }
          (_, _) => {
            v = [col[c - 1][0] + 1, buf[r - 1][c][1] + 1];
          }
        }
        col.push(v);
      });
      buf.push(col);
    });
    let mut ret: usize = 0;
    (0..grid.len()).for_each(|r| {
      (0..grid[0].len()).for_each(|c| {
        let mut size: usize = r + 1;
        if size > c + 1 {
          size = c + 1;
        }
        if size > buf[r][c][0] {
          size = buf[r][c][0];
        }
        if size > buf[r][c][1] {
          size = buf[r][c][1];
        }

        let mut cret: usize = 0;
        (0..size).rev().for_each(|l| {
          if buf[r][c - l][1] >= l + 1 && buf[r - l][c][0] >= l + 1 && l + 1 > cret {
            cret = l + 1;
          }
        });
        if cret > ret {
          ret = cret;
        }
      });
    });
    (ret * ret) as i32
  }
}
