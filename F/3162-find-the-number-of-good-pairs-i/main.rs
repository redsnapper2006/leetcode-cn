struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    nums2.iter().for_each(|&v| {
      let mut b = v * k;
      while b <= 50 {
        m.entry(b).and_modify(|x| *x += 1).or_insert(1);
        b += v * k;
      }
    });

    nums1
      .iter()
      .map(|x| {
        if m.contains_key(x) {
          *m.get(x).unwrap()
        } else {
          0
        }
      })
      .sum()
  }
}
