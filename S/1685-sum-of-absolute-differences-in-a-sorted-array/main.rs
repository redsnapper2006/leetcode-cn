struct Solution {}

impl Solution {
  pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
    let mut buf: Vec<i32> = Vec::new();
    let base = nums[0];
    let mut sum = 0;
    for i in 1..nums.len() {
      sum += nums[i] - base;
    }
    buf.push(sum);
    for i in 1..nums.len() {
      sum += (nums[i] - nums[i - 1]) * i as i32 - (nums.len() - i) as i32 * (nums[i] - nums[i - 1]);
      buf.push(sum);
    }
    buf
  }
}

fn main() {
  println!("{}", Solution::get_sum_absolute_differences(vec![0; 5]));
}
