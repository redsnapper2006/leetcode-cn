impl Solution {
  pub fn min_initial_strength(monsters: Vec<i32>, boosts: Vec<Vec<i32>>) -> i64 {
    let mut buf: Vec<i64> = vec![0; monsters.len() + 1];
    boosts.iter().for_each(|b| {
      buf[b[0] as usize] += b[2] as i64;
      buf[b[1] as usize + 1] -= b[2] as i64;
    });
    for i in 1..buf.len() {
      buf[i] += buf[i - 1];
    }

    let mut need: i64 = i64::MIN;
    for i in (0..monsters.len()).rev() {
      if need <= 0 {
        need = (monsters[i] as i64 - buf[i]).max(0);
      } else {
        need += monsters[i] as i64;
      }
    }

    need
  }
}
