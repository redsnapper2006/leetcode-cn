use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; move_time[0].len()]; move_time.len()];
    dp[0][0] = 0;
    let mut bh: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
    bh.push(Reverse((0, 0, 0)));

    while bh.len() > 0 {
      let (time, row, col) = bh.pop().unwrap().0;
      if row == move_time.len() - 1 && col == move_time[0].len() - 1 {
        return time;
      }
      if time > dp[row][col] {
        continue;
      }
      for cord in vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]] {
        let nr = row as i32 + cord[0];
        let nc = col as i32 + cord[1];
        if nr < 0 || nr >= move_time.len() as i32 || nc < 0 || nc >= move_time[0].len() as i32 {
          continue;
        }
        let nr = nr as usize;
        let nc = nc as usize;
        let v = time.max(move_time[nr][nc]) + 2 - (nr + nc) as i32 % 2;
        if v < dp[nr][nc] {
          dp[nr][nc] = v;
          bh.push(Reverse((v, nr as usize, nc as usize)));
        }
      }
    }

    -1
  }
}
