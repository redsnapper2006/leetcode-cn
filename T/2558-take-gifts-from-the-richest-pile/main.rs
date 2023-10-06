struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
  pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    gifts.iter().for_each(|&v| {
      heap.push(v);
    });
    (0..k).for_each(|_| {
      let v = heap.pop().unwrap();
      let remain = (v as f64).sqrt() as i32;
      heap.push(remain);
    });

    heap.iter().map(|v| *v as i64).sum::<i64>()
  }
}
