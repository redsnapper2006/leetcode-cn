impl Solution {
  pub fn even_odd_bit(n: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![0, 0];
    let mut n = n;
    let mut i: usize = 0;
    while n > 0 {
      ret[i] += n & 1;
      i ^= 1;
      n >>= 1;
    }
    ret
  }
}
