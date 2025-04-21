impl Solution {
  pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<i32> = vec![0; n as usize + 1];
    let mut offers = offers;
    offers.sort_unstable();
    let mut offer_idx: usize = 0;
    (0..=n as usize).for_each(|idx| {
      let prev = if idx > 0 { dp[idx - 1] } else { 0 };
      while offer_idx < offers.len() && offers[offer_idx][0] == idx as i32 {
        let next = offers[offer_idx][1] as usize;
        dp[next] = dp[next].max(prev + offers[offer_idx][2]);
        offer_idx += 1;
      }
      dp[idx] = dp[idx].max(prev);
    });

    dp[n as usize]
  }
}
