impl Solution {
  pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let k = k as usize;
    nums.sort_unstable();
    (k - 1..nums.len()).fold(i32::MAX, |ans, idx| ans.min(nums[idx] - nums[idx + 1 - k]))
  }
}
