impl Solution {
  pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    let size: usize = nums.len();
    nums.iter().enumerate().for_each(|(i, v)| {
      if size % (i + 1) == 0 {
        sum += v * v;
      }
    });
    sum
  }
}
