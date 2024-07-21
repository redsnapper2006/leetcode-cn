struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let mut dp0: i32 = arr[0];
    let mut dp1: i32 = 0;
    let mut ans: i32 = arr[0];

    (1..arr.len()).for_each(|idx| {
      dp1 = dp0.max(dp1 + arr[idx]);
      dp0 = dp0.max(0) + arr[idx];
      ans = ans.max(dp0.max(dp1));
    });

    ans
  }

  pub fn maximum_sum2(arr: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![0; arr.len()];
    let mut right: Vec<i32> = vec![0; arr.len()];
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    heap.push(Reverse(0));
    let mut sum: i32 = 0;
    (0..arr.len()).for_each(|idx| {
      sum += arr[idx];
      left[idx] = sum - heap.peek().unwrap().0;
      heap.push(Reverse(sum));
    });

    heap.clear();
    heap.push(Reverse(0));
    sum = 0;
    (0..arr.len()).for_each(|idx| {
      sum += arr[arr.len() - 1 - idx];
      right[arr.len() - 1 - idx] = sum - heap.peek().unwrap().0;
      heap.push(Reverse(sum));
    });

    let mut max: i32 = sum;
    (0..arr.len()).for_each(|idx| {
      max = max.max(left[idx] + right[idx] - arr[idx]);
      if left[idx] != arr[idx] || right[idx] != arr[idx] {
        max = max.max(left[idx] + right[idx] - arr[idx] * 2);
      }
    });
    max
  }
}
