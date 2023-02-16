struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|n| *m.entry(*n).or_insert(0) += 1);

    let (p, o) = m
      .iter()
      .fold((0, 0), |(pair, odd), (_, v)| (pair + v / 2, odd + v % 2));
    vec![p, o]
  }
}
