impl Solution {
  pub fn min_price(prices: Vec<i32>, discounts: Vec<i32>) -> f64 {
    let mut prices = prices;
    let mut discounts = discounts;
    prices.sort_unstable_by(|x, y| y.cmp(&x));
    discounts.sort_unstable_by(|x, y| y.cmp(&x));
    let mut ans: f64 = 0.0;
    for i in 0..prices.len() {
      ans += prices[i] as f64
        * (100 as f64 - if i < discounts.len() { discounts[i] } else { 0 } as f64)
        / 100 as f64;
    }
    ans
  }
}
