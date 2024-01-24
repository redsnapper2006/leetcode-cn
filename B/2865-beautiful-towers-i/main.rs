struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
  pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
    let mut left: Vec<i64> = vec![0; max_heights.len()];
    let mut right: Vec<i64> = vec![0; max_heights.len()];

    let mut stack: Vec<(i32, i32)> = Vec::new();
    (0..max_heights.len()).for_each(|i| {
      while stack.len() > 0 && stack[stack.len() - 1].0 >= max_heights[i] {
        stack.pop();
      }

      let mut offset: i32 = -1;
      let mut sum: i64 = 0;
      if stack.len() > 0 {
        offset = stack[stack.len() - 1].1;
        sum = left[offset as usize];
      }
      left[i] = sum + (i as i64 - offset as i64) * max_heights[i] as i64;
      stack.push((max_heights[i], i as i32));
    });

    stack.clear();
    (0..max_heights.len()).rev().for_each(|i| {
      while stack.len() > 0 && stack[stack.len() - 1].0 >= max_heights[i] {
        stack.pop();
      }

      let mut offset: i32 = max_heights.len() as i32;
      let mut sum: i64 = 0;
      if stack.len() > 0 {
        offset = stack[stack.len() - 1].1;
        sum = right[offset as usize];
      }

      right[i] = sum + (offset as i64 - i as i64) * max_heights[i] as i64;
      stack.push((max_heights[i], i as i32));
    });

    let mut max: i64 = 0;
    (0..max_heights.len()).rev().for_each(|i| {
      max = max.max(left[i] + right[i] - max_heights[i] as i64);
    });
    max
  }

  pub fn maximum_sum_of_heights2(max_heights: Vec<i32>) -> i64 {
    let mut left: Vec<i64> = vec![0; max_heights.len()];
    let mut right: Vec<i64> = vec![0; max_heights.len()];

    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    (0..max_heights.len()).for_each(|i| {
      while heap.len() > 0 && heap.peek().unwrap().0 >= max_heights[i] {
        heap.pop();
      }

      let mut offset: i32 = -1;
      let mut sum: i64 = 0;
      if heap.len() > 0 {
        offset = heap.peek().unwrap().1;
        sum = left[offset as usize];
      }
      left[i] = sum + (i as i64 - offset as i64) * max_heights[i] as i64;
      heap.push((max_heights[i], i as i32));
    });

    heap.clear();
    (0..max_heights.len()).rev().for_each(|i| {
      while heap.len() > 0 && heap.peek().unwrap().0 >= max_heights[i] {
        heap.pop();
      }

      let mut offset: i32 = max_heights.len() as i32;
      let mut sum: i64 = 0;
      if heap.len() > 0 {
        offset = heap.peek().unwrap().1;
        sum = right[offset as usize];
      }

      right[i] = sum + (offset as i64 - i as i64) * max_heights[i] as i64;
      heap.push((max_heights[i], i as i32));
    });

    let mut max: i64 = 0;
    (0..max_heights.len()).rev().for_each(|i| {
      max = max.max(left[i] + right[i] - max_heights[i] as i64);
    });
    max
  }
}
