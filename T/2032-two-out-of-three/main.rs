struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();

    for n in nums1 {
      let mut v = m.entry(n).or_insert(0);
      *v |= 1;
    }

    for n in nums2 {
      let mut v = m.entry(n).or_insert(0);
      *v |= 2;
    }

    for n in nums3 {
      let mut v = m.entry(n).or_insert(0);
      *v |= 4;
    }
    let mut ret: Vec<i32> = Vec::new();
    for (k, v) in m {
      if v.count_ones() > 1 {
        ret.push(k);
      }
    }
    ret
  }
}
