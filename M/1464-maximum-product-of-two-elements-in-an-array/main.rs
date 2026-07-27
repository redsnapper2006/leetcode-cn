impl Solution {
  pub fn max_product(nums: Vec<i32>) -> i32 {
    let (first, second) = nums.into_iter().fold((i32::MIN, i32::MIN), |(max1, max2), v| {
      if v > max1 {
        (v, max1)
      } else if v > max2 {
        (max1, v)
      } else {
        (max1, max2)
      }
    });
    (first - 1) * (second - 1)
  }
}
