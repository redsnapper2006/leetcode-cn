struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
    let mut m: HashSet<i32> = nums.into_iter().collect::<HashSet<i32>>();

    (0..move_from.len()).for_each(|idx| {
      m.remove(&move_from[idx]);
      m.insert(move_to[idx]);
    });
    let mut ans = m.into_iter().collect::<Vec<i32>>();
    ans.sort();
    ans
  }
}
