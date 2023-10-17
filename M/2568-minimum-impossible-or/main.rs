struct Solution;

impl Solution {
  pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    nums.iter().for_each(|&v| {
      if v & (v - 1) == 0 {
        sum |= v;
      }
    });

    let mut idx: i32 = 0;
    while idx < 32 {
      if 1 << idx & sum == 0 {
        return 1 << idx;
      }
      idx += 1;
    }
    1 << 31
  }
}

fn main() {
  println!("{}", Solution::min_impossible_or(vec![2, 1]));
}
