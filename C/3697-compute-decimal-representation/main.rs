impl Solution {
  pub fn decimal_representation(n: i32) -> Vec<i32> {
    let mut n = n;
    let mut buf: Vec<i32> = vec![];
    while n > 0 {
      buf.push(n % 10);
      n /= 10;
    }
    let mut base: i32 = 1;
    for i in 0..buf.len() {
      buf[i] *= base;
      base *= 10;
    }
    let mut s: usize = 0;
    let mut e: usize = buf.len() - 1;
    while s < e {
      let t = buf[s];
      buf[s] = buf[e];
      buf[e] = t;
      s += 1;
      e -= 1;
    }
    buf.retain(|x| *x != 0);
    buf
  }
}
