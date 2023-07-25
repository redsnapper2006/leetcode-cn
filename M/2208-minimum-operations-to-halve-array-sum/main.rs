struct Solution {}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct MaxNonNan(f64);

impl PartialOrd for MaxNonNan {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl Ord for MaxNonNan {
  fn cmp(&self, other: &MaxNonNan) -> Ordering {
    self.0.partial_cmp(&other.0).unwrap()
  }
}

impl PartialEq for MaxNonNan {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl Eq for MaxNonNan {}

impl Solution {
  pub fn halve_array(nums: Vec<i32>) -> i32 {
    let target = nums.iter().map(|&v| v as i64).sum::<i64>() as f64 / 2.0;
    let mut max_heap: BinaryHeap<MaxNonNan> = BinaryHeap::new();
    nums.iter().for_each(|&v| {
      max_heap.push(MaxNonNan(v as f64));
    });

    let mut cnt: i32 = 0;
    let mut sum: f64 = 0.0;
    while sum < target {
      let v = max_heap.pop().unwrap().0;
      sum += v / 2.0;
      max_heap.push(MaxNonNan(v / 2.0));
      cnt += 1;
    }
    cnt
  }
}
