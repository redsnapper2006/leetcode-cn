struct Solution {}

impl Solution {
  pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; 2]; prices.len()];

    buf[0][0] = -prices[0];
    (1..prices.len()).for_each(|idx| {
      buf[idx][0] = buf[idx - 1][0].max(buf[idx - 1][1] - prices[idx]);
      buf[idx][1] = buf[idx - 1][1].max(buf[idx - 1][0] + prices[idx] - fee);
    });

    buf[prices.len() - 1][1].max(buf[prices.len() - 1][0])
  }
}
