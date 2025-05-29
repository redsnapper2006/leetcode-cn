impl Solution {
  pub fn subarray_sum(nums: Vec<i32>) -> i32 {
    (0..nums.len())
      .fold((0, vec![0; nums.len()]), |(ans, mut sums), idx| {
        sums[idx] = if idx > 0 { sums[idx - 1] } else { 0 } + nums[idx];
        (
          ans
            + if idx as i32 > nums[idx] {
              sums[idx] - sums[idx - nums[idx] as usize - 1]
            } else {
              sums[idx]
            },
          sums,
        )
      })
      .0
  }
}
