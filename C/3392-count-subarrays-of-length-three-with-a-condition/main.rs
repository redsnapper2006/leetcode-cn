impl Solution {
  pub fn count_subarrays(nums: Vec<i32>) -> i32 {
    (1..nums.len() - 1).fold(0, |sum, idx| {
      sum
        + if nums[idx] == (nums[idx - 1] + nums[idx + 1]) * 2 {
          1
        } else {
          0
        }
    })
  }
}
