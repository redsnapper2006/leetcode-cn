impl Solution {
  pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut dp: Vec<i32> = vec![0; n];
    for i in 0..n {
      dp[i] = nums[i];
    }

    for i in (0..n - 1).rev() {
      for j in i + 1..n {
        dp[j] = (nums[i] - dp[j]).max(nums[j] - dp[j - 1]);
      }
    }
    dp[n - 1] >= 0
  }
}
