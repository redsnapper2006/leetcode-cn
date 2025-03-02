impl Solution {
  pub fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut dp: Vec<i32> = vec![-1; 1000001];
    let mut ans: i32 = cards.len() as i32 + 1;
    cards.iter().enumerate().for_each(|(idx, &v)| {
      let vv = v as usize;
      if dp[vv] != -1 {
        ans = ans.min(idx as i32 - dp[vv] + 1);
      }
      dp[vv] = idx as i32;
    });
    if ans == cards.len() as i32 + 1 {
      -1
    } else {
      ans
    }
  }
}
