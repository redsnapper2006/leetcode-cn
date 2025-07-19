use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
  pub fn minimum_difference(nums: Vec<i32>) -> i64 {
    let n = nums.len() / 3;
    let mut left: Vec<i64> = vec![0; nums.len()];
    let mut right: Vec<i64> = vec![0; nums.len()];
    let mut left_heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

    let mut sum: i64 = 0;
    for i in 0..nums.len() {
      let v = nums[i] as i64;
      sum += v;
      left_heap.push(v);
      while left_heap.len() > n {
        let vv = left_heap.pop().unwrap();
        sum -= vv;
      }
      left[i] = sum;
    }
    sum = 0;
    for i in (0..nums.len()).rev() {
      let v = nums[i] as i64;
      sum += v;
      right_heap.push(Reverse(v));
      while right_heap.len() > n {
        let vv = right_heap.pop().unwrap().0;
        sum -= vv;
      }
      right[i] = sum;
    }

    let mut ans: i64 = i64::MAX;
    for i in n - 1..2 * n {
      ans = ans.min(left[i] - right[i + 1]);
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!("{} ", Solution::minimum_difference(vec![3, 1, 2]));
  println!("{} ", Solution::minimum_difference(vec![7, 9, 5, 8, 1, 3]));
  println!(
    "{} ",
    Solution::minimum_difference(vec![
      16, 46, 43, 41, 42, 14, 36, 49, 50, 28, 38, 25, 17, 5, 18, 11, 14, 21, 23, 39, 23
    ])
  );
}
