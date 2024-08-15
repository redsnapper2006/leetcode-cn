struct Solution {}

impl Solution {
  pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut ans: i32 = i32::MIN;
    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        let mut min: i32 = grid[row][col];
        if col > 0 {
          min = min.min(dp[row][col - 1]);
          ans = ans.max(grid[row][col] - dp[row][col - 1]);
        }
        if row > 0 {
          min = min.min(dp[row - 1][col]);
          ans = ans.max(grid[row][col] - dp[row - 1][col]);
        }
        dp[row][col] = min;
      });
    });

    ans
  }

  pub fn max_score2(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut ans: i32 = i32::MIN;
    (0..grid.len()).rev().for_each(|row| {
      (0..grid[0].len()).rev().for_each(|col| {
        let mut v: i32 = dp[row][col];
        if col < grid[0].len() - 1 {
          let diff = grid[row][col + 1] - grid[row][col];
          v = v.max(match dp[row][col + 1] > 0 {
            true => diff + dp[row][col + 1],
            _ => diff,
          });
        }
        if row < grid.len() - 1 {
          let diff = grid[row + 1][col] - grid[row][col];
          v = v.max(match dp[row + 1][col] > 0 {
            true => diff + dp[row + 1][col],
            _ => diff,
          });
        }
        dp[row][col] = v;
        ans = ans.max(v);
      });
    });

    match ans == 0 {
      true => {
        let mut diff: i32 = i32::MIN;
        (0..grid.len()).for_each(|row| {
          (1..grid[0].len()).for_each(|col| {
            diff = diff.max(grid[row][col] - grid[row][col - 1]);
          });
        });
        (0..grid[0].len()).for_each(|col| {
          (1..grid.len()).for_each(|row| {
            diff = diff.max(grid[row][col] - grid[row - 1][col]);
          });
        });
        diff
      }
      _ => ans,
    }
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_score(vec![
      vec![9, 5, 7, 3],
      vec![8, 9, 6, 1],
      vec![6, 7, 14, 3],
      vec![2, 5, 3, 1]
    ])
  );

  println!(
    "{}",
    Solution::max_score(vec![vec![4, 3, 2], vec![3, 2, 1],])
  );
}
