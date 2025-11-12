use std::collections::HashMap;

impl Solution {
  pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let mut left: HashMap<i32, i32> = HashMap::new();
    left.insert(0, -1);
    let mut sum: i32 = 0;
    let mut left_dp: Vec<i32> = vec![arr.len() as i32; arr.len()];
    let mut left_min: i32 = arr.len() as i32;
    (0..arr.len()).for_each(|idx| {
      sum += arr[idx];
      if left.contains_key(&(sum - target)) {
        left_min = left_min.min(idx as i32 - left.get(&(sum - target)).unwrap());
      }
      left_dp[idx] = left_min;
      left.insert(sum, idx as i32);
    });

    let mut right: HashMap<i32, i32> = HashMap::new();
    right.insert(0, arr.len() as i32);
    sum = 0;
    let mut right_dp: Vec<i32> = vec![arr.len() as i32; arr.len()];
    let mut right_min: i32 = arr.len() as i32;
    (0..arr.len()).rev().for_each(|idx| {
      sum += arr[idx];
      if right.contains_key(&(sum - target)) {
        right_min = right_min.min(right.get(&(sum - target)).unwrap() - idx as i32);
      }
      right_dp[idx] = right_min;
      right.insert(sum, idx as i32);
    });
    let mut ans: i32 = arr.len() as i32 + 1;
    (0..arr.len() - 1).for_each(|idx| {
      ans = ans.min(left_dp[idx] + right_dp[idx + 1]);
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
