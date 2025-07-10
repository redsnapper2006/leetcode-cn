use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
  pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut heap  : BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let bb = colors.as_bytes().to_vec();
    let mut base : u8 = bb[0];
    heap.push(Reverse(needed_time[0]));
    let mut ans : i32 = 0;
    for i in 1..bb.len() {
      if bb[i] != base {
        while heap.len() > 1 {
          ans += heap.pop().unwrap().0;
        }
        heap.pop();
      }
      base = bb[i];
      heap.push(Reverse(needed_time[i]));
    }
    while heap.len() > 1 {
      ans += heap.pop().unwrap().0;
    }

    ans
  }
}
