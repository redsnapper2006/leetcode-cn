struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();

    for (idx, v) in nums.iter().enumerate() {
      let diff = target - v;
      if m.contains_key(&diff) {
        return vec![m[&diff] as i32, idx as i32];
      }
      m.insert(*v, idx);
    }
    vec![]
  }
}
