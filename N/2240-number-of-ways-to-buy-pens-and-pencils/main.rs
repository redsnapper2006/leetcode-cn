struct Solution {}

impl Solution {
  pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let mut res: i64 = 0;

    let mut start: i32 = 0;
    while start <= total {
      res += ((total - start) / cost2) as i64 + 1;
      start += cost1;
    }
    res
  }
}
