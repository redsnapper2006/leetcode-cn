struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn minimum_coins(prices: Vec<i32>) -> i32 {
    let mut h: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    h.push(Reverse((0, prices.len())));

    let mut ans: i32 = 0;
    (0..prices.len()).rev().for_each(|idx| {
      let start = idx + 1;
      let end = (idx + 1) * 2;
      fn is_in(idx: usize, start: usize, end: usize) -> bool {
        idx >= start && idx <= end
      }
      while h.len() > 0 && !is_in(h.peek().unwrap().0 .1, start, end) {
        h.pop();
      }
      ans = h.peek().unwrap().0 .0 + prices[idx];
      h.push(Reverse((ans, idx)));
    });

    ans
  }
}
