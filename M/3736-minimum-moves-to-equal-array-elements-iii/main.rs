impl Solution {
  pub fn min_moves(nums: Vec<i32>) -> i32 {
    *nums.iter().max().unwrap() * nums.len() as i32 - nums.iter().sum::<i32>()
  }
}
