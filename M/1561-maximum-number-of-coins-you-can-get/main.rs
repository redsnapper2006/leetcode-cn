struct Solution {}

impl Solution {
  pub fn max_coins(piles: Vec<i32>) -> i32 {
    let mut piles = piles;
    piles.sort_unstable();
    piles[piles.len() / 3..]
      .iter()
      .enumerate()
      .fold(0, |sum, (idx, v)| if idx % 2 == 0 { sum + v } else { sum })
  }
}
