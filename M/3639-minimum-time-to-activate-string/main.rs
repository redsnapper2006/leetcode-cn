impl Solution {
  pub fn min_time(s: String, order: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    let mut arr: Vec<i64> = vec![-1, s.len() as i64];
    let mut aggr: i64 = 0;
    let mut idx: usize = 0;
    while idx < order.len() {
      let n = order[idx] as i64;
      let ll = match arr.binary_search(&n) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      arr.insert(ll, n);

      aggr += (arr[ll] - arr[ll - 1]) * (arr[ll + 1] - arr[ll]);
      if aggr >= k {
        break;
      }
      idx += 1;
    }
    if aggr < k { -1 } else { idx as _ }
  }
}
