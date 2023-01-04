struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn find_subarrays(nums: Vec<i32>) -> bool {
    let mut hs: HashSet<i32> = HashSet::new();
    hs.insert(nums[0] + nums[1]);

    for i in 1..nums.len() - 1 {
      let v = nums[i] + nums[i + 1];
      if hs.contains(&v) {
        return true;
      }
      hs.insert(v);
    }
    false
  }
}
