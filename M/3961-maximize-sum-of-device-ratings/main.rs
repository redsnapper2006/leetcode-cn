impl Solution {
  pub fn max_ratings(units: Vec<Vec<i32>>) -> i64 {
    if units[0].len() == 1 {
      return units.iter().fold(0i64, |ans, u| ans + u[0] as i64);
    }

    let mut units = units;
    for u in &mut units {
      u.sort_unstable();
    }
    let mut ans: i64 = 0;
    let mut mn1: i64 = i64::MAX;
    let mut mn2: i64 = i64::MAX;
    for i in 0..units.len() {
      ans += units[i][1] as i64;
      mn2 = mn2.min(units[i][1] as i64);
      mn1 = mn1.min(units[i][0] as i64);
    }
    ans - mn2 + mn1
  }
}
