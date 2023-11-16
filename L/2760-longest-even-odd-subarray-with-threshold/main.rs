struct Solution {}

impl Solution {
  pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let mut start: usize = 0;
    let mut max: i32 = 0;
    let mut idx: usize = 0;
    while idx < nums.len() {
      if nums[idx] % 2 == 1 || nums[idx] > threshold {
        idx += 1;
        continue;
      }

      let start = idx;
      while idx < nums.len() {
        if nums[idx] % 2 == (idx - start) as i32 % 2 && nums[idx] <= threshold {
          idx += 1;
        } else {
          break;
        }
      }
      if max < (idx - start) as i32 {
        max = (idx - start) as i32;
      }
    }
    max
  }
}
