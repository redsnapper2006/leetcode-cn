impl Solution {
  pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; 11]; n as usize];
    pick.iter().for_each(|p| {
      dp[p[0] as usize][p[1] as usize] += 1;
    });
    dp.iter().enumerate().fold(0, |sum, (idx, r)| {
      sum
        + if (*r.iter().max().unwrap() > idx as i32) {
          1
        } else {
          0
        }
    })
  }
}
