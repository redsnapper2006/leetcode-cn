impl Solution {
  pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<(i32, i32)> = vec![(-1, -1); 100001];
    let mut ans: i32 = i32::MAX;
    nums.iter().enumerate().for_each(|(idx, &v)| {
      let v = v as usize;
      if dp[v].0 == -1 {
        dp[v].0 = idx as i32;
      } else if dp[v].1 == -1 {
        dp[v].1 = idx as i32;
      } else {
        ans = ans.min((idx as i32 - dp[v].0) * 2);
        dp[v].0 = dp[v].1;
        dp[v].1 = idx as i32;
      }
    });
    if ans == i32::MAX { -1 } else { ans }
  }
}
