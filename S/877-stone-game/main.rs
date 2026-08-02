impl Solution {
  pub fn stone_game(piles: Vec<i32>) -> bool {
    true
  }

  pub fn stone_game_dp(piles: Vec<i32>) -> bool {
    let n = piles.len();
    let mut dp: Vec<i32> = vec![0; n];
    for i in 0..n {
      dp[i] = piles[i];
    }

    for i in (0..n - 1).rev() {
      for j in i + 1..n {
        dp[j] = (piles[i] - dp[j]).max(piles[j] - dp[j - 1]);
      }
    }
    dp[n - 1] >= 0
  }
}
