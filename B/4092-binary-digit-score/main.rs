impl Solution {
  pub fn calculate_score(a: i32, b: i32) -> i32 {
    let calc = |v: i32| -> i32 {
      let mut v = v;
      let mut s = 0i32;
      while v > 0 {
        s += v & 1;
        v >>= 1;
      }
      s
    };
    let s1 = calc(a);
    let s2 = calc(b);
    if s1 > s2 { s1 * s2 } else { s2 / s1 }
  }
}
