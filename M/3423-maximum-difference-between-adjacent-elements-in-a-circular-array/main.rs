impl Solution {
  pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    (0..nums.len()).fold(i32::MIN, |max, idx| {
      max.max((nums[idx] - nums[(idx + 1) % nums.len()]).abs())
    })
  }
}
