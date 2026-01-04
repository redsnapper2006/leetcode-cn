impl Solution {
  pub fn maximize_expression_of_three(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len() - 1] + nums[nums.len() - 2] - nums[0]
  }
}
