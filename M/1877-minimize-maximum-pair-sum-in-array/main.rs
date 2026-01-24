impl Solution {
  pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    (0..nums.len() / 2).fold(0, |ans, idx| {
      ans.max(nums[idx] + nums[nums.len() - 1 - idx])
    })
  }
}
