impl Solution {
  pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.clone().iter_mut().fold(0, |sum, n| {
      while *n >= 100 {
        *n /= 100;
      }
      sum + if *n > 9 { 1 } else { 0 }
    })
  }
}

struct Solution {}
