struct Solution {}

impl Solution {
  pub fn minimum_average(nums: Vec<i32>) -> f64 {
    let mut nums = nums;
    nums.sort_unstable();
    (0..nums.len() / 2).fold(100.0 as f64, |min_v, idx| {
      min_v.min((nums[idx] + nums[nums.len() - 1 - idx]) as f64 / 2.0)
    })
  }
}
