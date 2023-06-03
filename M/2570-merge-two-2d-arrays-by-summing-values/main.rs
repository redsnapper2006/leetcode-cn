use std::iter::FromIterator;

struct Solution {}

impl Solution {
  pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut i1: usize = 0;
    let mut i2: usize = 0;
    let mut res: Vec<Vec<i32>> = Vec::new();

    while i1 < nums1.len() && i2 < nums2.len() {
      if nums1[i1][0] < nums2[i2][0] {
        res.push(nums1[i1].clone());
        i1 += 1;
      } else if nums1[i1][0] > nums2[i2][0] {
        res.push(nums2[i2].clone());
        i2 += 1;
      } else {
        res.push(vec![nums1[i1][0], nums1[i1][1] + nums2[i2][1]]);
        i1 += 1;
        i2 += 1;
      }
    }
    if i1 < nums1.len() {
      res.append(&mut Vec::from_iter(nums1[i1..].iter().cloned()));
    }
    if i2 < nums2.len() {
      res.append(&mut Vec::from_iter(nums2[i2..].iter().cloned()));
    }

    res
  }
}
