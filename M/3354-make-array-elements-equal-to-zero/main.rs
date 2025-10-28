impl Solution {
  pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
    let total = nums.iter().sum::<i32>();
    nums
      .iter()
      .fold((0, 0), |(ans, sum), &v| {
        (
          ans
            + if v == 0 {
              (2 - ((sum + v) * 2 - total).abs()).max(0)
            } else {
              0
            },
          sum + v,
        )
      })
      .0
  }
}
