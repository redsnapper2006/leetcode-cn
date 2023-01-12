struct Solution {}

impl Solution {
  pub fn maximum_xor(nums: Vec<i32>) -> i32 {
    nums.into_iter().reduce(|acc, x| acc | x).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::maximum_xor(vec![772, 45, 297, 549, 549, 301, 805, 297])
  );
}
