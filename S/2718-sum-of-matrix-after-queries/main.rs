use std::collections::HashSet;
impl Solution {
  pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
    let mut r = n as i64;
    let mut c = n as i64;
    let mut sum: i64 = 0;
    let mut rs: HashSet<i32> = HashSet::new();
    let mut cs: HashSet<i32> = HashSet::new();

    for i in (0..queries.len()).rev() {
      let t = queries[i][0];
      let ii = queries[i][1];
      let v = queries[i][2] as i64;

      if t == 0 && !rs.contains(&ii) {
        rs.insert(ii);
        sum += v * c;
        r -= 1;
      }
      if t == 1 && !cs.contains(&ii) {
        cs.insert(ii);
        sum += v * r;
        c -= 1;
      }
    }
    sum
  }
}
