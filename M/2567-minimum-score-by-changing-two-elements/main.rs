impl Solution {
  pub fn minimize_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();
    (nums[n - 1] - nums[2]).min(nums[n - 2] - nums[1]).min((nums[n - 3] - nums[0]))
  }
}
