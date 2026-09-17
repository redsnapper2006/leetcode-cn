use std::collections::HashMap;

impl Solution {
  pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0usize, 0usize);
    let mut pre_min: Vec<usize> = vec![arr.len(); arr.len() + 1];
    let mut sum = 0i32;
    let mut min_len = arr.len();
    let mut ans = arr.len() + 1;
    (0..arr.len()).for_each(|right| {
      sum += arr[right];

      while left < right && sum > target {
        sum -= arr[left];
        left += 1;
      }
      if sum == target {
        ans = ans.min(pre_min[left] + right - left + 1);
        min_len = min_len.min(right - left + 1);
      }
      pre_min[right + 1] = min_len;
    });

    if ans == arr.len() + 1 { -1 } else { ans as i32 }
  }

  pub fn min_sum_of_lengths2(arr: Vec<i32>, target: i32) -> i32 {
    let mut left: HashMap<i32, i32> = HashMap::new();
    left.insert(0, -1);
    let mut sum: i32 = 0;
    let mut left_dp: Vec<i32> = vec![arr.len() as i32; arr.len()];
    (0..arr.len()).for_each(|idx| {
      sum += arr[idx];
      if idx > 0 {
        left_dp[idx] = left_dp[idx - 1];
      }
      let diff = sum - target;
      if let Some(prev) = left.get(&diff) {
        left_dp[idx] = left_dp[idx].min(idx as i32 - prev);
      }
      left.insert(sum, idx as i32);
    });

    let mut ans: i32 = arr.len() as i32 + 1;
    let mut right: HashMap<i32, i32> = HashMap::new();
    right.insert(0, arr.len() as i32);
    sum = 0;
    let mut right_min: i32 = arr.len() as i32;
    (0..arr.len()).rev().for_each(|idx| {
      sum += arr[idx];
      let diff = sum - target;
      if let Some(next) = right.get(&diff) {
        right_min = right_min.min(next - idx as i32);
      }
      if idx > 0 {
        ans = ans.min(left_dp[idx - 1] + right_min);
      }
      right.insert(sum, idx as i32);
    });

    if ans == (arr.len() as i32 + 1) {
      -1
    } else {
      ans
    }
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3));
  println!(
    "{}",
    Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6)
  );
  println!("{}", Solution::min_sum_of_lengths(vec![5, 5, 4, 4, 5], 3));
  println!(
    "{}",
    Solution::min_sum_of_lengths(vec![3, 1, 1, 1, 5, 1, 2, 1], 3)
  );
}
