impl Solution {
  pub fn min_moves(nums: Vec<i32>) -> i32 {
    let mx = *nums.iter().max().unwrap();
    nums.iter().fold(0, |sum, v| sum + mx - v)
  }
}
