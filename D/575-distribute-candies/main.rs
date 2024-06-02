struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    HashSet::<_>::from_iter(candy_type.iter().copied())
      .len()
      .min(candy_type.len() / 2) as i32
  }
}
