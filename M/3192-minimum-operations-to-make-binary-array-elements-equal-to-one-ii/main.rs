struct Solution {}

impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |ans, v| ans + (ans + v + 1) % 2)
  }
}
