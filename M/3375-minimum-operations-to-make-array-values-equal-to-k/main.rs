struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut vs: Vec<i32> = HashSet::<i32>::from_iter(nums.iter().cloned()).into_iter().collect();
    vs.sort_unstable();
    if vs[0] < k {
      -1
    } else if vs[0] == k {
      vs.len() as i32 - 1
    } else {
      vs.len() as i32
    }
  }
}

fn main() {
  println!("{}", Solution::min_operations(vec![5, 2, 4, 5, 5], 3));
}
