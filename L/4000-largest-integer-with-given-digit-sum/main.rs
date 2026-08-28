impl Solution {
  pub fn largest_integer(n: i32, s: i32) -> i32 {
    let mut n = n;
    let mut s = s;
    let mut ans: i32 = 0;
    while n > 0 {
      ans = ans * 10 + s.min(9).max(0);
      n -= 1;
      s -= 9;
    }
    if s > 0 { -1 } else { ans }
  }
}
