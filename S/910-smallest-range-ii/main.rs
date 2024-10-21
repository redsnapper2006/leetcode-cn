struct Solution {}

impl Solution {
  pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut ans: i32 = nums[nums.len() - 1] - nums[0];
    (1..nums.len()).for_each(|idx| {
      ans = ans
        .min((nums[idx - 1] + k).max(nums[nums.len() - 1] - k) - (nums[0] + k).min(nums[idx] - k));
    });
    ans
  }
}
