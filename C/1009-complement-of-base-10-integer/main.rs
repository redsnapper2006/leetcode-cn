impl Solution {
  pub fn bitwise_complement(n: i32) -> i32 {
    if n == 0 {
      return 1;
    }
    let mut mask = n;
    mask |= mask >> 1;
    mask |= mask >> 2;
    mask |= mask >> 4;
    mask |= mask >> 8;
    mask |= mask >> 16;
    n ^ mask
  }

  pub fn bitwise_complement2(n: i32) -> i32 {
    if n == 0 {
      return 1;
    }
    n ^ ((1 << (32 - n.leading_zeros())) - 1)
  }
}
