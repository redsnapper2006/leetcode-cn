struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    l.into_iter()
      .zip(r.into_iter())
      .collect::<Vec<(i32, i32)>>()
      .iter()
      .map(|&(left, right)| {
        // println!("{} {}", left, right);
        let mut min: i32 = nums[left as usize];
        let mut max: i32 = nums[left as usize];
        let mut m: HashSet<i32> = HashSet::new();
        (left..=right).for_each(|idx| {
          if min > nums[idx as usize] {
            min = nums[idx as usize];
          }
          if max < nums[idx as usize] {
            max = nums[idx as usize];
          }
          m.insert(nums[idx as usize]);
        });
        if (max - min) % (right - left) != 0 {
          return false;
        }
        let diff = (max - min) / (right - left);
        if diff == 0 {
          return m.len() == 1;
        }
        let mut start = min;
        while start <= max {
          if !m.contains(&start) {
            return false;
          }
          start += diff;
        }
        true
      })
      .collect::<Vec<bool>>()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::check_arithmetic_subarrays(
      vec![-3, -6, -8, -4, -2, -8, -6, 0, 0, 0, 0],
      vec![5, 4, 3, 2, 4, 7, 6, 1, 7],
      vec![6, 5, 6, 3, 7, 10, 7, 4, 10]
    )
  );
}
