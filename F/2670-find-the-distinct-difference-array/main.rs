struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ls: HashSet<i32> = HashSet::new();
    let mut rs: HashSet<i32> = HashSet::new();
    let mut ret: Vec<i32> = vec![0; nums.len()];
    (0..nums.len()).for_each(|i| {
      ret[nums.len() - 1 - i] -= rs.len() as i32;
      ls.insert(nums[i]);
      rs.insert(nums[nums.len() - 1 - i]);
      ret[i] += ls.len() as i32;
    });

    ret
  }
}
