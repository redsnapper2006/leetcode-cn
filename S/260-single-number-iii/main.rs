struct Solution {}

impl Solution {
  pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut sum: i32 = 0;
    nums.iter().for_each(|&x| sum ^= x);

    let flag = (-sum) & sum;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    nums.iter().for_each(|&x| {
      if flag & x == 0 {
        a ^= x;
      } else {
        b ^= x;
      }
    });
    vec![a, b]
  }
}
