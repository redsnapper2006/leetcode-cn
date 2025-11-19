use std::collections::HashSet;

impl Solution {
  pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
    let mut m: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    let mut o = original;
    while m.contains(&o) {
      o *= 2;
    }
    o
  }
}
