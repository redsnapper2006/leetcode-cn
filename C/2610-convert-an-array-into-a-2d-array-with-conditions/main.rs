struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|&x| *cnt.entry(x).or_insert(0) += 1);
    let mut res: Vec<Vec<i32>> = Vec::new();

    while !cnt.is_empty() {
      let mut t: Vec<i32> = Vec::new();
      cnt.iter_mut().for_each(|(k, v)| {
        t.push(k);
        *v -= 1;
      });
      cnt.retain(|_, v| *v > 0);
      res.push(t);
    }
    res
  }
}
