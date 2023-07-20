struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut min_h: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut max_h: BinaryHeap<i32> = BinaryHeap::new();

    let total: i32 = nums.iter().sum();
    min_h.push(Reverse(0));
    max_h.push(nums[0]);

    let mut min: i32 = 1 << 31 - 1;
    let mut max: i32 = -1 << 31;
    let mut sum: i32 = 0;
    nums.iter().for_each(|&v| {
      sum += v;

      let min_t = min_h.peek().unwrap().0;
      if sum - min_t > max {
        max = sum - min_t;
      }
      let max_t = max_h.peek().unwrap();
      if sum - max_t < min {
        min = sum - max_t;
      }
      min_h.push(Reverse(sum));
      max_h.push(sum);
    });

    if max > total - min {
      max
    } else {
      total - min
    }
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_subarray_sum_circular(vec![1, -2, 3, -2])
  );
}
