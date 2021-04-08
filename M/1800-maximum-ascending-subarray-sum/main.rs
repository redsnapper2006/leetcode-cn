struct Solution {}

impl Solution {
  pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut max: i32 = nums[0];
    let mut sum: i32 = nums[0];
    for i in 1..nums.len() {
      if nums[i] > nums[i - 1] {
        sum += nums[i];
      } else {
        if sum > max {
          max = sum;
        }
        sum = nums[i];
      }
    }
    if sum > max {
      max = sum;
    }
    max
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12])
  );
}
