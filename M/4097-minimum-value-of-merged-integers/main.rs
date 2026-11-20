impl Solution {
  pub fn min_merged_integer(nums: Vec<i32>) -> i32 {
    (0..nums.len())
      .step_by(2)
      .map(|i| format!("{}{}", nums[i], nums[i + 1]).parse::<i32>().unwrap())
      .min()
      .unwrap()
  }
}
