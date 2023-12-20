struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut m1: HashMap<i32, usize> = HashMap::new();
    let mut m2: HashMap<i32, usize> = HashMap::new();

    let mut r1: i32 = 0;
    let mut r2: i32 = 0;

    nums1.iter().enumerate().for_each(|(idx, n)| {
      let c = m1.entry(*n).or_insert(idx);
      *c = idx;
    });

    nums2.iter().enumerate().for_each(|(idx, n)| {
      let c = m2.entry(*n).or_insert(idx);
      *c = idx;
    });

    nums1.iter().enumerate().for_each(|(idx, n)| {
      if m2.contains_key(n) {
        r1 += 1;
      }
    });
    nums2.iter().enumerate().for_each(|(idx, n)| {
      if m1.contains_key(n) {
        r2 += 1;
      }
    });
    vec![r1, r2]
  }
}
