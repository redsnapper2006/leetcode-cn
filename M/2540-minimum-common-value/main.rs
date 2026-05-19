use std::cmp::Ordering;

impl Solution {
  pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut idx1, mut idx2): (usize, usize) = (0, 0);

    while idx1 < nums1.len() && idx2 < nums2.len() {
      match nums1[idx1].cmp(&nums2[idx2]) {
        Ordering::Equal => {
          return nums1[idx1];
        }
        Ordering::Greater => {
          idx2 += 1;
        }
        _ => {
          idx1 += 1;
        }
      }
    }
    -1
  }
}

struct Solution {}
