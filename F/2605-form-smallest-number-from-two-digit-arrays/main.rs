struct Solution {}

use std::cmp::Ordering;
impl Solution {
  pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ns1 = nums1;
    ns1.sort();
    let mut ns2 = nums2;
    ns2.sort();
    let mut cnt: [i32; 9] = [0; 9];
    ns1.iter().for_each(|v| cnt[(v - 1) as usize] += 1);
    ns2.iter().for_each(|v| cnt[(v - 1) as usize] += 1);
    let mut idx: usize = 0;
    while idx < 9 {
      if cnt[idx] == 2 {
        return idx as i32 + 1;
      }
      idx += 1;
    }

    match ns1[0].cmp(&ns2[0]) {
      Ordering::Greater => ns2[0] * 10 + ns1[0],
      _ => ns1[0] * 10 + ns2[0],
    }
  }
}
