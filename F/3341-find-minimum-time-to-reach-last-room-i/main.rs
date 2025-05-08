use std::collections::VecDeque;
impl Solution {
  pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; move_time[0].len()]; move_time.len()];
    dp[0][0] = 0;
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();
    q.push_back((0, 0, 0));
    while q.len() > 0 {
      let (row, col, step) = q.pop_front().unwrap();
      vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]]
        .iter()
        .for_each(|cord| {
          let nr = row as i32 + cord[0];
          let nc = col as i32 + cord[1];
          if nr < 0 || nr >= move_time.len() as i32 || nc < 0 || nc >= move_time[0].len() as i32 {
            return;
          }
          let nr = nr as usize;
          let nc = nc as usize;
          let v = step.max(move_time[nr][nc]) + 1;
          if v < dp[nr][nc] {
            dp[nr][nc] = v;
            q.push_back((nr as usize, nc as usize, v));
          }
        });
    }

    dp[move_time.len() - 1][move_time[0].len() - 1]
  }
}
