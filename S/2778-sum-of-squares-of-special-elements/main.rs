impl Solution {
  pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .enumerate()
      .fold((0i32, nums.len()), |(ans, n), (i, v)| {
        (ans + if n % (i + 1) == 0 { v * v } else { 0 }, n)
      })
      .0
  }
}
