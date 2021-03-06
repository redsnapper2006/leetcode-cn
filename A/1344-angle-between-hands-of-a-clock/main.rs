struct Solution {}

impl Solution {
  pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let mut r: f64 =
      30 as f64 * (hour as f64) + minutes as f64 / 2 as f64 - 6 as f64 * (minutes as f64);
    if r < 0.0 {
      r = -r;
    }
    let rr: f64 = 360 as f64 - r;
    if rr > r {
      return r;
    }
    rr
  }
}

fn main() {
  println!("{}", Solution::angle_clock(10, 20));
}
