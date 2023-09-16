struct Solution {}

impl Solution {
  pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
      return nums[0];
    }
    let mut dp: Vec<i32> = vec![0; nums.len()];
    dp[0] = nums[0];
    dp[1] = nums[1];
    (2..nums.len()).for_each(|idx| {
      let mut max = dp[idx - 2];
      if idx > 2 {
        max = max.max(dp[idx - 3]);
      }

      dp[idx] = nums[idx] + max;
    });
    dp[nums.len() - 1].max(dp[nums.len() - 2])
  }
}
