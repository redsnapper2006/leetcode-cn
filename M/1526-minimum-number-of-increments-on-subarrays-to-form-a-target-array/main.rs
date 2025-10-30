impl Solution {
  pub fn min_number_operations(target: Vec<i32>) -> i32 {
    target
      .iter()
      .fold((0, 0), |(ans, prev), &v| (ans + (v - prev).max(0), v))
      .0
  }
}
