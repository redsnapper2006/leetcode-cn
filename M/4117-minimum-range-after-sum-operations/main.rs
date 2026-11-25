impl Solution {
  pub fn min_range(a: i32, b: i32, c: i32) -> i32 {
    let mn = a.min(b).min(c);
    let mx = a.max(b).max(c);
    (mx - mn).min(a + b + c - mn - mx)
  }
}
