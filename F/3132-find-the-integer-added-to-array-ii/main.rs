struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    nums1.sort_unstable();
    nums2.sort_unstable();
    let mut m1: HashMap<i32, i32> = HashMap::new();
    let mut m2: HashMap<i32, i32> = HashMap::new();
    nums1.iter().for_each(|&v| {
      m1.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    nums2.iter().for_each(|&v| {
      m2.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    nums1.dedup();
    nums2.dedup();
    let size = nums2.len();
    let mut i1: usize = nums1.len() - 1;
    while i1 > 0 {
      let diff = nums1[i1] - nums2[size - 1];
      let mut i2: usize = 0;
      let mut is_found: bool = true;
      while i2 < nums2.len() {
        let v = nums2[i2] + diff;
        if !m1.contains_key(&v) || m1.get(&v).unwrap() < m2.get(&nums2[i2]).unwrap() {
          is_found = false;
          break;
        }
        i2 += 1;
      }
      if is_found {
        return -diff;
      }
      i1 -= 1;
    }
    nums2[size - 1] - nums1[i1]
  }
}

fn main() {
  println!(
    "{}",
    Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10])
  );
  println!(
    "{}",
    Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7])
  );
}
