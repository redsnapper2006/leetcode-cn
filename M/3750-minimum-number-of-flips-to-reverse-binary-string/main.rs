impl Solution {
  pub fn minimum_flips(n: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];
    let mut n = n;
    while n > 0 {
      buf.push(n % 2);
      n /= 2;
    }
    let mut s: usize = 0;
    let mut e: usize = buf.len() - 1;
    let mut ans: i32 = 0;
    while s < e {
      ans += if buf[s] != buf[e] { 2 } else { 0 };
      s += 1;
      e -= 1;
    }
    ans
  }
}
