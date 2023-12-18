struct Solution {}

impl Solution {
  pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut e: i32 = nums.len() as i32 - 1;

    while s < e {
      let m = s + (e - s) / 2;
      if m == 0 {
        if nums[m as usize] > nums[(m + 1) as usize] {
          return m;
        } else {
          s = m + 1;
        }
      } else if m == nums.len() as i32 - 1 {
        if nums[m as usize] > nums[(m - 1) as usize] {
          return m;
        } else {
          e = m - 1;
        }
      } else {
        if nums[m as usize] > nums[(m - 1) as usize] && nums[m as usize] > nums[(m + 1) as usize] {
          return m;
        } else if nums[m as usize] < nums[(m - 1) as usize] {
          e = m - 1;
        } else {
          s = m + 1;
        }
      }
    }
    s
  }
}
