use std::collections::HashSet;
impl Solution {
  pub fn max_sum(nums: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    let mut v = set.into_iter().collect::<Vec<i32>>();
    v.sort_unstable();
    if v[v.len() - 1] < 0 {
      return v[v.len() - 1];
    }
    v.into_iter()
      .filter(|x| *x > 0)
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}
