struct Solution {}

use std::collections::VecDeque;
impl Solution {
  pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();

    let mut fresh: bool = false;
    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        if grid[row][col] == 2 {
          q.push_back((row, col, 0));
        }
        if grid[row][col] == 1 {
          fresh = true;
        }
      });
    });

    if !fresh {
      return 0;
    }

    if q.len() == 0 {
      return -1;
    }

    let mut ans: i32 = 0;

    while q.len() > 0 {
      let (row, col, step) = q.pop_front().unwrap();
      if step > ans {
        ans = step;
      }
      if row > 0 && grid[row - 1][col] == 1 {
        grid[row - 1][col] = 2;

        q.push_back((row - 1, col, step + 1));
      }
      if row < grid.len() - 1 && grid[row + 1][col] == 1 {
        grid[row + 1][col] = 2;
        q.push_back((row + 1, col, step + 1));
      }
      if col > 0 && grid[row][col - 1] == 1 {
        grid[row][col - 1] = 2;
        q.push_back((row, col - 1, step + 1));
      }
      if col < grid[0].len() - 1 && grid[row][col + 1] == 1 {
        grid[row][col + 1] = 2;
        q.push_back((row, col + 1, step + 1));
      }
    }

    fresh = false;
    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        if grid[row][col] == 1 {
          fresh = true;
        }
      });
    });

    match fresh {
      true => -1,
      _ => ans,
    }
  }
}
