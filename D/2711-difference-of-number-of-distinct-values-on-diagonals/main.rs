use std::collections::HashSet;
impl Solution {
  pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut tlc: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut brc: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut tls: Vec<Vec<HashSet<i32>>> = vec![vec![HashSet::new(); grid[0].len()]; grid.len()];
    let mut brs: Vec<Vec<HashSet<i32>>> = vec![vec![HashSet::new(); grid[0].len()]; grid.len()];

    let mut ans: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        tlc[row][col] = match (row, col) {
          (0, _) => 0,
          (_, 0) => 0,
          (_, _) => {
            tls[row][col] = tls[row - 1][col - 1].clone();
            tls[row - 1][col - 1].len() as i32
          }
        };
        tls[row][col].insert(grid[row][col]);
      });
    });

    (0..grid.len()).rev().for_each(|row| {
      (0..grid[0].len()).rev().for_each(|col| {
        brc[row][col] = if row == grid.len() - 1 || col == grid[0].len() - 1 {
          0
        } else {
          brs[row][col] = brs[row + 1][col + 1].clone();
          brs[row + 1][col + 1].len() as i32
        };
        brs[row][col].insert(grid[row][col]);
      });
    });

    (0..grid.len()).for_each(|row| {
      (0..grid[0].len()).for_each(|col| {
        ans[row][col] = (tlc[row][col] - brc[row][col]).abs();
      });
    });

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::difference_of_distinct_values(vec![vec![1, 2, 3], vec![3, 1, 5], vec![3, 2, 1]])
  );
}
