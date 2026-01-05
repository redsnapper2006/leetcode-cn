impl Solution {
  pub fn abs_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    (0..k as usize).fold(0, |ans, idx| ans + nums[nums.len() - 1 - idx] - nums[idx])
  }
}
