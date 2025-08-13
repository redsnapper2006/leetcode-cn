impl Solution {
  pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let b1 = text1.as_bytes().to_vec();
    let b2 = text2.as_bytes().to_vec();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; b2.len()]; b1.len()];
    for i in 0..b1.len() {
      if i == 0 {
        dp[i][0] = if b1[i] == b2[0] { 1 } else { 0 };
      } else {
        dp[i][0] = dp[i - 1][0].max(if b1[i] == b2[0] { 1 } else { 0 });
      }
    }
    for i in 0..b2.len() {
      if i == 0 {
        dp[0][i] = if b1[0] == b2[i] { 1 } else { 0 };
      } else {
        dp[0][i] = dp[0][i - 1].max(if b1[0] == b2[i] { 1 } else { 0 });
      }
    }
    for i in 1..b1.len() {
      for j in 1..b2.len() {
        dp[i][j] = (dp[i - 1][j - 1] + if b1[i] == b2[j] { 1 } else { 0 })
          .max(dp[i - 1][j])
          .max(dp[i][j - 1]);
      }
    }

    dp[b1.len() - 1][b2.len() - 1]
  }
}

struct Solution {}
fn main() {
  println!(
    "{}",
    Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string())
  );

  println!(
    "{}",
    Solution::longest_common_subsequence("abc".to_string(), "abc".to_string())
  );

  println!(
    "{}",
    Solution::longest_common_subsequence("abc".to_string(), "def".to_string())
  );
}
