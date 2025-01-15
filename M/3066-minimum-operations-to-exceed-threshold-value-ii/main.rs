struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut bh: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    nums.iter().for_each(|&v| {
      bh.push(Reverse(v as i64));
    });

    let mut ans: i32 = 0;
    while bh.peek().unwrap().0 < k as i64 {
      let c1 = bh.pop().unwrap().0;
      let c2 = bh.pop().unwrap().0;
      bh.push(Reverse(c1 * 2 + c2));
      ans += 1;
    }
    ans
  }
}
