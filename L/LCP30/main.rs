struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn magic_tower(nums: Vec<i32>) -> i32 {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let mut blood: i64 = 1;
    let mut remain: Vec<i32> = Vec::new();
    let mut ret: i32 = 0;
    for i in 0..nums.len() {
      if nums[i] < 0 {
        heap.push(Reverse(nums[i]));
      }
      blood += nums[i] as i64;

      while blood <= 0 && !heap.is_empty() {
        let max = heap.pop().unwrap().0;
        blood -= max as i64;
        remain.push(max);
        ret += 1;
      }
      if blood <= 0 {
        return -1;
      }
    }

    for i in 0..remain.len() {
      blood += remain[i] as i64;
      if blood <= 0 {
        return -1;
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::magic_tower(vec![100, 100, 100, -250, -60, -140, -50, -50, 100, 150])
  );
}
