impl Solution {
  pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let step = (sx - fx).abs().max((sy - fy).abs());
    if step == 0 {
      t != 1
    } else {
      step <= t
    }
  }
}
