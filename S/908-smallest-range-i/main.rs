struct Solution {}

impl Solution {
  pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let (max, min) = nums.iter().fold((i32::MIN, i32::MAX), |(max, min), &v| {
      (max.max(v), min.min(v))
    });
    if max - min >= 2 * k {
      max - min - 2 * k
    } else {
      0
    }
  }
}
