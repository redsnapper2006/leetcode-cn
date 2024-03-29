impl Solution {
  pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![0; nums.len()];
    let mut right: Vec<i32> = vec![0; nums.len()];

    let mut base = nums[0];
    (1..nums.len()).for_each(|idx| {
      base = base.min(nums[idx]);
      left[idx] = base;
    });

    let mut base = nums[nums.len() - 1];
    (0..nums.len() - 1).rev().for_each(|idx| {
      base = base.min(nums[idx]);
      right[idx] = base;
    });

    let mut min: i32 = i32::MAX;
    (1..nums.len() - 1).for_each(|idx| {
      if left[idx] != nums[idx] && right[idx] != nums[idx] {
        min = min.min(left[idx] + nums[idx] + right[idx]);
      }
    });
    match min {
      i32::MAX => -1,
      _ => min,
    }
  }
}
