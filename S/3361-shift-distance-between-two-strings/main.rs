impl Solution {
  pub fn shift_distance(s: String, t: String, next_cost: Vec<i32>, previous_cost: Vec<i32>) -> i64 {
    let sb = s.as_bytes();
    let tb = t.as_bytes();
    let nc_sum = next_cost.iter().map(|&x| x as i64).sum::<i64>() as i64;
    let pc_sum = previous_cost.iter().map(|&x| x as i64).sum::<i64>() as i64;
    let mut ans: i64 = 0;
    for i in 0..sb.len() {
      let off1 = (sb[i] - b'a') as usize;
      let off2 = (tb[i] - b'a') as usize;

      if off1 <= off2 {
        let mut n_sum: i64 = 0;
        for ii in off1..off2 {
          n_sum += next_cost[ii] as i64;
        }
        let mut p_sum: i64 = 0;
        for ii in off1 + 1..=off2 {
          p_sum += previous_cost[ii] as i64;
        }
        ans += n_sum.min(pc_sum - p_sum);
      } else {
        let mut n_sum: i64 = 0;
        for ii in off2..off1 {
          n_sum += next_cost[ii] as i64;
        }
        let mut p_sum: i64 = 0;
        for ii in off2 + 1..=off1 {
          p_sum += previous_cost[ii] as i64;
        }
        ans += (nc_sum - n_sum).min(p_sum);
      }
    }
    ans
  }
}
