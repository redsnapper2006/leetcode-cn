struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let set: HashSet<i32> = nums.clone().into_iter().collect::<HashSet<i32>>();
    nums
      .iter()
      .map(
        |&v| match set.contains(&(v + diff)) && set.contains(&(v + diff + diff)) {
          true => 1,
          _ => 0,
        },
      )
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}

fn main() {
  println!("{}", Solution::arithmetic_triplets(vec![1, 2, 3], 1));
}
