struct Solution {}

impl Solution {
  pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut buf = nums.to_vec();
    buf.sort();
    let mut sum = 0;
    for i in (0..buf.len()).step_by(2) {
      sum += buf[i];
    }
    sum
  }
}

fn main() {
  println!("{}", Solution::array_pair_sum(vec![0; 5]));
}
