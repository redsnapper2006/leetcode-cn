impl Solution {
  pub fn min_cutting_cost(n: i32, m: i32, k: i32) -> i64 {
    let mut cost: i64 = 0;
    let n = n as i64;
    let m = m as i64;
    let k = k as i64;
    if n > k {
      cost += k * (n - k);
    }
    if m > k {
      cost += k * (m - k);
    }
    cost
  }
}
