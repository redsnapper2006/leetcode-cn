struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
  pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let mut max_heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    let mut left: Vec<i64> = vec![0; max_heights.len()];
    let mut right: Vec<i64> = vec![0; max_heights.len()];

    let mut idx: i32 = 0;
    while idx < max_heights.len() as i32 {
      while max_heap.len() > 0 && max_heap.peek().unwrap().0 >= max_heights[idx as usize] {
        max_heap.pop();
      }

      let mut start: i32 = -1;
      let mut prev: i64 = 0;
      if max_heap.len() > 0 {
        start = max_heap.peek().unwrap().1;
        prev = left[start as usize];
      }
      left[idx as usize] = prev + (idx as i64 - start as i64) * max_heights[idx as usize] as i64;
      max_heap.push((max_heights[idx as usize], idx));
      idx += 1;
    }

    let mut idx: i32 = max_heights.len() as i32 - 1;
    max_heap.clear();
    while idx >= 0 {
      while max_heap.len() > 0 && max_heap.peek().unwrap().0 >= max_heights[idx as usize] {
        max_heap.pop();
      }

      let mut start: i32 = max_heights.len() as i32;
      let mut prev: i64 = 0;
      if max_heap.len() > 0 {
        start = max_heap.peek().unwrap().1 as i32;
        prev = right[start as usize];
      }

      right[idx as usize] = prev + (start as i64 - idx as i64) * max_heights[idx as usize] as i64;
      max_heap.push((max_heights[idx as usize], idx));
      idx -= 1;
    }

    let mut max: i64 = 0;
    (0..max_heights.len()).for_each(|i| {
      max = std::cmp::max(max, left[i] + right[i] - max_heights[i] as i64);
    });
    max
  }
}
