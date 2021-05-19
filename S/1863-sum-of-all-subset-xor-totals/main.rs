struct Solution {}

impl Solution {
  pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![0, nums[0]];
    for i in 1..nums.len() {
      let size = buf.len();
      for j in 0..size {
        buf.push(buf[j] ^ nums[i]);
      }
    }
    let mut sum: i32 = 0;
    for i in 0..buf.len() {
      sum = sum + buf[i];
    }
    sum
  }
}

fn main() {
  println!("{}", Solution::subset_xor_sum(vec![0; 5]));
}
