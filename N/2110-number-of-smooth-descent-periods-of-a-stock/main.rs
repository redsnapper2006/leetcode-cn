struct Solution {}

impl Solution {
  pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut res: i64 = 0;
    while end < prices.len() {
      if prices[end] != prices[end - 1] - 1 {
        res += ((end - start) * (end - start + 1) / 2) as i64 ;
        start = end;
      }
      end += 1;
    }
    res + ((end - start) * (end - start + 1) / 2) as i64
  }
}
