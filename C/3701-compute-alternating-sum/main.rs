impl Solution {
  pub fn alternating_sum(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .enumerate()
      .fold(0, |sum, (idx, &v)| sum + if idx % 2 == 0 { v } else { -v })
  }
}
