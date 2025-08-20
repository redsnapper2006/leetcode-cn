impl Solution {
  pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];
    let mut sum: i32 = 0;
    for i in 0..matrix.len() {
      for j in 0..matrix[0].len() {
        if matrix[i][j] == 0 {
          continue;
        }
        dp[i][j] = (if i > 0 { dp[i - 1][j] } else { 0 })
          .min(if j > 0 { dp[i][j - 1] } else { 0 })
          .min(if i > 0 && j > 0 { dp[i - 1][j - 1] } else { 0 })
          + 1;
        sum += dp[i][j];
      }
    }
    sum
  }
}
