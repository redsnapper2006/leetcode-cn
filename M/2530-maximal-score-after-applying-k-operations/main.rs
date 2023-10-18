struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
  pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
    let mut buf: BinaryHeap<i32> = BinaryHeap::new();
    nums.iter().for_each(|&v| {
      buf.push(v);
    });

    let mut sum: i64 = 0;
    (0..k).for_each(|_| {
      let v = buf.pop().unwrap();
      sum += v as i64;
      buf.push((v + 2) / 3);
    });

    sum
  }
}

fn main() {
  println!("{}", Solution::max_kelements(vec![1, 10, 3, 3, 3], 3));
}
