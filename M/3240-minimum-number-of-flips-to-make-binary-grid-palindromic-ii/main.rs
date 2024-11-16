struct Solution {}

impl Solution {
  pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
    let rs = grid.len();
    let cs = grid[0].len();

    let mut ans: i32 = 0;
    (0..rs / 2).for_each(|row| {
      (0..cs / 2).for_each(|col| {
        let mut sum: i32 = grid[row][col]
          + grid[row][cs - 1 - col]
          + grid[rs - 1 - row][col]
          + grid[rs - 1 - row][cs - 1 - col];
        ans += sum.min(4 - sum);
      });
    });
    if rs % 2 == 1 && cs % 2 == 1 {
      ans += grid[rs / 2][cs / 2];
    }

    let mut diff = 0;
    let mut cnt1 = 0;
    if rs % 2 == 1 {
      for j in 0..cs / 2 {
        if grid[rs / 2][j] != grid[rs / 2][cs - 1 - j] {
          diff += 1;
        } else {
          cnt1 += grid[rs / 2][j] * 2;
        }
      }
    }
    if cs % 2 == 1 {
      for i in 0..rs / 2 {
        if grid[i][cs / 2] != grid[rs - 1 - i][cs / 2] {
          diff += 1;
        } else {
          cnt1 += grid[i][cs / 2] * 2;
        }
      }
    }

    ans + if diff != 0 { diff } else { cnt1 % 4 }
  }
}
