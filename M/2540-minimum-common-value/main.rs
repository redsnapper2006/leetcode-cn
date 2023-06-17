struct Solution {}

impl Solution {
  pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut idx1: usize = 0;
    let mut idx2: usize = 0;

    while idx1 < nums1.len() && idx2 < nums2.len() {
      if nums1[idx1] == nums2[idx2] {
        return nums1[idx1];
      } else if nums1[idx1] > nums2[idx2] {
        idx2 += 1;
      } else {
        idx1 += 1;
      }
    }
    -1
  }
}
