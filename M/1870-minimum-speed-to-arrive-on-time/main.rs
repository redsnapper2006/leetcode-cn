struct Solution {}

impl Solution {
  pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
    let mut s: i32 = 1;
    let sum: i32 = dist.iter().sum::<i32>();
    let mut e: i32 = 1000000001;
    while s <= e {
      let m = s + (e - s) / 2;
      let mut sum: i32 = 0;
      let mut idx: usize = 0;
      while idx < dist.len() - 1 {
        sum += (dist[idx] + m - 1) / m;
        idx += 1;
      }
      let mut sum: f64 = sum as f64 + dist[dist.len() - 1] as f64 / m as f64;
      if sum <= hour {
        e = m - 1;
      } else {
        s = m + 1;
      }
    }
    match s > 1000000001 {
      true => -1,
      _ => s,
    }
  }
}
