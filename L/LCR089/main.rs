impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; nums.len()];

    dp[0][0] = nums[0];
    (1..nums.len()).for_each(|idx| {
      dp[idx][0] = dp[idx - 1][1] + nums[idx];
      dp[idx][1] = dp[idx - 1][0].max(dp[idx - 1][1]);
    });

    dp[nums.len() - 1][0].max(dp[nums.len() - 1][1])
  }
}
