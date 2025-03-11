struct Solution {}

impl Solution {
  pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![0; nums.len()];
    left[0] = nums[0];
    let mut right: Vec<i32> = vec![0; nums.len()];
    right[nums.len() - 1] = nums[nums.len() - 1];
    (1..nums.len() - 1).for_each(|idx| {
      left[idx] = left[idx - 1].max(nums[idx]);
      right[nums.len() - 1 - idx] = right[nums.len() - idx].min(nums[nums.len() - 1 - idx]);
    });

    (1..nums.len() - 1)
      .map(|idx| {
        match (
          nums[idx] > left[idx - 1] && nums[idx] < right[idx + 1],
          nums[idx] > nums[idx - 1] && nums[idx] < nums[idx + 1],
        ) {
          (true, _) => 2,
          (false, true) => 1,
          (_, _) => 0,
        }
      })
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}
