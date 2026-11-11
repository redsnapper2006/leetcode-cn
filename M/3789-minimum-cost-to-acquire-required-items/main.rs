impl Solution {
  pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
    let (mut cost1, mut cost2, cost_both) = (cost1 as i64, cost2 as i64, cost_both as i64);
    let (mut need1, mut need2) = (need1 as i64, need2 as i64);
    if need1 > need2 {
      std::mem::swap(&mut need1, &mut need2);
      std::mem::swap(&mut cost1, &mut cost2);
    }

    let v = need1 * cost_both + (need2 - need1) * cost2;
    v.min(cost1 * need1 + cost2 * need2).min(need2 * cost_both)
  }
}
