impl Solution {
  pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<(usize, usize)> = vec![(nums.len(), nums.len()); 101];
    let mut ans: usize = nums.len() * 2;
    nums.iter().enumerate().for_each(|(idx, &n)| {
      let n = n as usize;
      if dp[n].0 == nums.len() {
        dp[n].0 = idx;
      } else if dp[n].1 == nums.len() {
        dp[n].1 = idx;
      } else {
        ans = ans.min((idx - dp[n].0) * 2);
        dp[n].0 = dp[n].1;
        dp[n].1 = idx;
      }
    });

    if ans == nums.len() * 2 { -1 } else { ans as _ }
  }
}
