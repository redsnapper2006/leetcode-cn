struct Solution {}

impl Solution {
  pub fn valid_partition(nums: Vec<i32>) -> bool {
    let mut dp: Vec<bool> = vec![false; nums.len() + 1];
    dp[0] = true;

    (0..nums.len()).for_each(|idx| {
      if !dp[idx] {
        return;
      }
      if idx + 1 < nums.len() && !dp[idx + 2] {
        dp[idx + 2] = nums[idx] == nums[idx + 1];
      }

      if idx + 2 < nums.len() && !dp[idx + 3] {
        dp[idx + 3] = nums[idx] == nums[idx + 1] && nums[idx + 1] == nums[idx + 2]
          || nums[idx] + 1 == nums[idx + 1] && nums[idx + 1] + 1 == nums[idx + 2];
      }
    });

    dp[nums.len()]
  }
}
