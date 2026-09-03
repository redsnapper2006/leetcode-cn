impl Solution {
  pub fn maximum_points(enemy_energies: Vec<i32>, current_energy: i32) -> i64 {
    let mut total: i64 = 0;
    let mut mn: i64 = i64::MAX;
    for &n in &enemy_energies {
      total += n as i64;
      mn = mn.min(n as i64);
    }
    let ce = current_energy as i64;
    if ce < mn {
      0
    } else {
      (total - mn + ce - mn) / mn + 1
    }
  }
}
