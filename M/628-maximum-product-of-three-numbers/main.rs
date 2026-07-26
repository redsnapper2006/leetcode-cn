impl Solution {
  pub fn maximum_product(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let n = nums.len();
    (nums[n - 1] * nums[n - 2] * nums[n - 3]).max(nums[n - 1] * nums[1] * nums[0])
  }
}
