impl Solution {
  pub fn dominant_indices(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .enumerate()
      .rev()
      .fold((0, 0.0), |(cnt, sum), (idx, &v)| {
        (
          cnt
            + if idx < nums.len() - 1 && (sum + v as f64) / ((nums.len() - idx) as f64) < v as f64 {
              1
            } else {
              0
            },
          sum + v as f64,
        )
      })
      .0
  }
}
