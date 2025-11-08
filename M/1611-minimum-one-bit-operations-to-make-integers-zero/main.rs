impl Solution {
  pub fn minimum_one_bit_operations(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut n = n;
    (0..32).for_each(|offset| {
      let bit = n & (1 << offset);
      if bit == 0 {
        return;
      }
      ans = (bit << 1) - 1 - ans;
      n -= bit;
    });
    ans
  }
}
