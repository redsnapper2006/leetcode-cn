use std::collections::HashSet;

impl Solution {
  pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    let mut b: i32 = k;
    while set.contains(&b) {
      b += k;
    }
    b
  }
}
