struct Solution {}

impl Solution {
  pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let nn = nums.len();
    let mut min: usize = 0;
    let mut max: usize = nums.len();
    nums.iter().enumerate().for_each(|(idx, &v)| {
      if v == 1 {
        min = idx;
      }
      if v == nn as i32 {
        max = idx;
      }
    });
    (min + nn - 1 - max - (min > max) as usize) as i32
  }
}
