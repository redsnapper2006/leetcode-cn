struct Solution {}

impl Solution {
  pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
    let mut pp = prices;
    pp.sort();
    match pp[0] + pp[1] > money {
      true => money,
      _ => money - pp[0] - pp[1],
    }
  }
}
