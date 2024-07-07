impl Solution {
  pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut vis: i64 = 0;
    let mut ans: i32 = 0;
    nums.iter().for_each(|&v| {
      if vis & (1 << v) > 0 {
        ans ^= v;
      }

      vis |= 1 << v;
    });
    ans
  }
}
