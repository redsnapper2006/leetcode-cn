struct Solution {}

impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut ret: i32 = 0;
    (0..32).for_each(|i| {
      let mut sum: i32 = 0;
      nums.iter().for_each(|&v| {
        sum += (v >> i) & 1;
      });
      if sum % 3 != 0 {
        ret += 1 << i;
      }
    });
    ret
  }
}
