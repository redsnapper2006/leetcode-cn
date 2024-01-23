struct Solution {}

impl Solution {
  pub fn minimum_cost(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut sum: i32 = nums[0];
    nums.remove(0);
    nums.sort();
    sum + nums[0] + nums[1]
  }
}
