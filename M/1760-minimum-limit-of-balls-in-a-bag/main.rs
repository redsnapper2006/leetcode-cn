struct Solution {}

impl Solution {
  pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
    let mut left: i32 = 1;
    let mut right: i32 = *nums.iter().max().unwrap();

    let mut ret: i32 = 0;
    while left <= right {
      let m: i32 = left + (right - left) / 2;
      let mut ops: i32 = 0;
      for n in &nums {
        ops += (n - 1) / m;
      }
      if ops <= max_operations {
        right = m - 1;
        ret = m;
      } else {
        left = m + 1;
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::minimum_size(vec![2, 4, 8, 2], 4));
}
