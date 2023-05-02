use std::collections::HashSet;
impl Solution {
  pub fn distinct_averages(nums: Vec<i32>) -> i32 {
    let mut m = nums;
    m.sort();
    let mut set: HashSet<i32> = HashSet::new();
    (0..m.len() / 2).for_each(|idx| {
      set.insert(m[idx] + m[m.len() - 1 - idx]);
    });
    set.len() as i32
  }
}
