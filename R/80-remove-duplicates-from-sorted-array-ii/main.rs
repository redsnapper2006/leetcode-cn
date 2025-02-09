struct Solution {}

impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
      return nums.len() as _;
    }
    let mut start: usize = 2;
    let mut end: usize = 2;
    (2..nums.len()).for_each(|idx| {
      if nums[start - 2] != nums[idx] {
        nums[start] = nums[idx];
        start += 1;
      }
    });
    start as _
  }
}
