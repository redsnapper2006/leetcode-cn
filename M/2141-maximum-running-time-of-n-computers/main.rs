impl Solution {
  pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let n = n as i64;
    let total = batteries.iter().map(|x| *x as i64).sum::<i64>();

    let mut l: i64 = 0;
    let mut r: i64 = total / n + 1;
    while l <= r {
      let m = l + (r - l) / 2;
      if batteries.iter().fold(0, |sum, &v| sum + m.min(v as i64)) >= m * n {
        l = m + 1;
      } else {
        r = m - 1;
      }
    }
    r
  }
}
