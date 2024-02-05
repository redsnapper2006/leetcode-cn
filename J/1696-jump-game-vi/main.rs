struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
  pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let mut dp: Vec<i32> = vec![0; nums.len()];
    dp[0] = nums[0];

    let mut max_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    max_heap.push((nums[0], 0));

    (1..nums.len()).for_each(|idx| {
      while max_heap.len() > 0 && max_heap.peek().unwrap().1 + (k as usize) < idx {
        max_heap.pop();
      }
      dp[idx] = max_heap.peek().unwrap().0 + nums[idx];
      max_heap.push((dp[idx], idx));
    });

    dp[nums.len() - 1]
  }
}

fn main() {
  println!("{}", Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2));

  println!(
    "{}",
    Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2)
  );
}
