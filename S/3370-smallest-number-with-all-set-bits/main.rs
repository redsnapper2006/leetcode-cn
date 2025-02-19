impl Solution {
  pub fn smallest_number(n: i32) -> i32 {
    let mut m = n;
    let mut offset: i32 = 0;
    while m > 0 {
      m /= 2;
      offset += 1;
    }

    ((1 << offset) - 1) | n
  }
}
