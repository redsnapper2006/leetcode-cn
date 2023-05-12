struct Solution {}

impl Solution {
  pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
    (nums.iter().sum::<i32>()
      - nums
        .iter()
        .map(|&v| {
          let mut vv = v;
          let mut sum: i32 = 0;
          while vv > 0 {
            sum += vv % 10;
            vv /= 10;
          }
          sum
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum::<i32>())
    .abs()
  }
}
