struct Solution {}

impl Solution {
  pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
    if (c - e).abs() == (d - f).abs() {
      if a != e && (d - f) / (c - e) == (b - f) / (a - e) && (a > c && a < e || a > e && a < c) {
        2
      } else {
        1
      }
    } else {
      if a == e && (c != a || d > b && d > f || d < b && d < f)
        || b == f && (d != b || c < a && c < e || c > a && c > e)
      {
        1
      } else {
        2
      }
    }
  }
}
