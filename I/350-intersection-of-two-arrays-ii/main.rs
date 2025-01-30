struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();

    let s: &Vec<i32> = if nums1.len() > nums2.len() {
      nums2.as_ref()
    } else {
      nums1.as_ref()
    };

    let l: &Vec<i32> = if nums1.len() > nums2.len() {
      nums1.as_ref()
    } else {
      nums2.as_ref()
    };

    s.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);
    });
    let mut ans: Vec<i32> = Vec::new();
    l.iter().for_each(|&v| {
      m.entry(v).and_modify(|x| {
        if *x > 0 {
          ans.push(v);
        }
        *x -= 1;
      });
    });
    ans
  }
}

fn main() {
  println!("{:?}", Solution::intersect(vec![1, 2, 3], vec![2, 3, 4]));
}
