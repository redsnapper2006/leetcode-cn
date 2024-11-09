struct Solution {}

impl Solution {
  pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = nums.len() / 2;
    while left < right {
      let m = left + (right - left) / 2;
      if nums[m * 2] != nums[m * 2 + 1] {
        right = m;
      } else {
        left = m + 1;
      }
    }
    nums[right * 2]
  }
}
