struct Solution {}

impl Solution {
  pub fn is_good(nums: Vec<i32>) -> bool {
    let mut n = nums;

    n.sort();

    let idx: usize = 0;
    while idx < n.len() - 1 {
      if n[idx] != idx + 1 {
        return false;
      }
      idx += 1;
    }
    n[idx] == n.len() - 1
  }
}
