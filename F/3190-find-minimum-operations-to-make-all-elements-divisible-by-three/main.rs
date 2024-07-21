impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |aggr, v| aggr + (v % 3).min(3 - v % 3))
  }
}
