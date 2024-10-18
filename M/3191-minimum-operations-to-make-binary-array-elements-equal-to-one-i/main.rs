struct Solution {}

impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    let mut idx: usize = 0;
    let mut ans: i32 = 0;
    while idx + 3 <= nums.len() {
      if nums[idx] == 0 {
        nums[idx] = 1 - nums[idx];
        nums[idx + 1] = 1 - nums[idx + 1];
        nums[idx + 2] = 1 - nums[idx + 2];
        ans += 1;
      }
      idx += 1;
    }

    match nums[idx - 1] == 1 && nums[idx] == 1 && nums[idx + 1] == 1 {
      true => ans,
      _ => -1,
    }
  }
}
