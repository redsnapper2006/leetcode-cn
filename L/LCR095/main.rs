impl Solution {
  pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let b1 = text1.as_bytes().to_vec();
    let b2 = text2.as_bytes().to_vec();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; b2.len()]; b1.len()];
    dp[0][0] = if b1[0] == b2[0] { 1 } else { 0 };
    for i in 1..b1.len() {
      dp[i][0] = if b1[i] == b2[0] { 1 } else { dp[i - 1][0] };
    }
    for i in 1..b2.len() {
      dp[0][i] = if b1[0] == b2[i] { 1 } else { dp[0][i - 1] };
    }
    for r in 1..b1.len() {
      for c in 1..b2.len() {
        dp[r][c] = dp[r - 1][c]
          .max(dp[r][c - 1])
          .max(dp[r - 1][c - 1] + if b1[r] == b2[c] { 1 } else { 0 });
      }
    }
    dp[b1.len() - 1][b2.len() - 1]
  }
}
