use std::collections::BinaryHeap;

impl Solution {
  pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
    let mut ladders = ladders;
    let mut idx: usize = 0;

    let mut sum: i32 = 0;
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    while idx < heights.len() - 1 {
      if heights[idx + 1] > heights[idx] {
        let diff = heights[idx + 1] - heights[idx];
        sum += diff;
        heap.push(diff);
        if sum > bricks {
          if ladders == 0 {
            break;
          }
          ladders -= 1;
          sum -= heap.pop().unwrap();
        }
      }

      idx += 1;
    }
    idx as i32 + 1
  }
}
