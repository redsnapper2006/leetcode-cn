struct Solution {}

use std::collections::VecDeque;
impl Solution {
  pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; grid[0].len()]; grid.len()];
    if grid[0][0] == 1 {
      return -1;
    }
    dp[0][0] = 1;

    let cord: Vec<(i32, i32)> = vec![
      (-1, -1),
      (-1, 0),
      (-1, 1),
      (0, -1),
      (0, 1),
      (1, -1),
      (1, 0),
      (1, 1),
    ];

    let mut buf: VecDeque<(usize, usize, i32)> = VecDeque::new();
    buf.push_back((0, 0, 1));
    while buf.len() > 0 {
      let (r, c, s) = buf.pop_front().unwrap();
      cord.iter().for_each(|(cr, cc)| {
        let nr = r as i32 + cr;
        let nc = c as i32 + cc;
        if nr < 0
          || nr >= grid.len() as i32
          || nc < 0
          || nc >= grid[0].len() as i32
          || grid[nr as usize][nc as usize] == 1
          || dp[nr as usize][nc as usize] != -1
        {
          return;
        }

        dp[nr as usize][nc as usize] = s + 1;
        buf.push_back((nr as usize, nc as usize, s + 1));
      });
    }

    dp[grid.len() - 1][grid[0].len() - 1]
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::shortest_path_binary_matrix(vec![
      vec![0, 1, 1, 0, 0, 0],
      vec![0, 1, 0, 1, 1, 0],
      vec![0, 1, 1, 0, 1, 0],
      vec![0, 0, 0, 1, 1, 0],
      vec![1, 1, 1, 1, 1, 0],
      vec![1, 1, 1, 1, 1, 0]
    ])
  );
}
