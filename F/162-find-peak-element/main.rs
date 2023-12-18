struct Solution {}

impl Solution {
  pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut s: i32 = 0;
    let mut e: i32 = nums.len() as i32 - 1;

    while s < e {
      let m = s + (e - s) / 2;
      if nums[m as usize] > nums[(m + 1) as usize] {
        e = m;
      } else {
        s = m+1;
      }
    }
    e
  }
}
