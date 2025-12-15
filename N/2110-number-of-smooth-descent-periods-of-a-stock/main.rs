struct Solution {}

impl Solution {
  pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    let mut cnt: i64 = 0;
    for i in 0..prices.len() {
      cnt = if i > 0 && prices[i] == prices[i - 1] - 1 {
        cnt + 1
      } else {
        1
      };
      ans += cnt;
    }
    ans
  }

  pub fn get_descent_periods2(prices: Vec<i32>) -> i64 {
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut res: i64 = 0;
    while end < prices.len() {
      if prices[end] != prices[end - 1] - 1 {
        res += ((end - start) * (end - start + 1) / 2) as i64;
        start = end;
      }
      end += 1;
    }
    res + ((end - start) * (end - start + 1) / 2) as i64
  }
}
