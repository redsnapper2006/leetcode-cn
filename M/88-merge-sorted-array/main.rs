struct Solution {}

impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut midx = m - 1;
    let mut nidx = n - 1;
    let mut idx = m + n - 1;
    while midx >= 0 || nidx >= 0 {
      if midx < 0 {
        nums1[idx as usize] = nums2[nidx as usize];
        nidx -= 1;
      } else if nidx < 0 {
        nums1[idx as usize] = nums1[midx as usize];
        midx -= 1;
      } else if nums1[midx as usize] > nums2[nidx as usize] {
        nums1[idx as usize] = nums1[midx as usize];
        midx -= 1;
      } else {
        nums1[idx as usize] = nums2[nidx as usize];
        nidx -= 1;
      }
      idx -= 1;
    }
  }
}
