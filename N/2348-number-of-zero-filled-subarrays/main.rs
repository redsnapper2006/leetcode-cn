impl Solution {
  pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    nums
      .iter()
      .fold((0, 0), |(ans, cnt), v| {
        (
          ans + if v == 0 { cnt + 1 } else { 0 },
          if v == 0 { cnt + 1 } else { 0 },
        )
      })
      .0
  }
}
