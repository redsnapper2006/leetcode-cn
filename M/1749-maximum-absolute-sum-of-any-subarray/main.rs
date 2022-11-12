struct Solution {}

impl Solution {
  pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut ret = 0;
    let mut min = 0;
    let mut max = 0;
    for i in 0..nums.len() {
      sum += nums[i];
      if sum > max {
        max = sum;
      }
      if sum < min {
        min = sum;
      }
      let diff = max - min;
      if diff > ret {
        ret = diff
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::max_absolute_sum(vec![0; 5]));
}
