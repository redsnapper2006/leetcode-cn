impl Solution {
  pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let h = hour as f64;
    let m = minutes as f64;
    let r: f64 = (30.0 * h - (6.0 - 0.5) * m).abs();
    (360.0 - r).min(r)
  }

  pub fn angle_clock2(hour: i32, minutes: i32) -> f64 {
    let mut r: f64 = 30 as f64 * (hour as f64) + minutes as f64 / 2 as f64 - 6 as f64 * (minutes as f64);
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

struct Solution {}

fn main() {
  println!("{}", Solution::angle_clock(10, 20));
}
