struct Solution {}

impl Solution {
  pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<(i32, i32)> = vec![(0, 0); n as usize + 1];
    trust.iter().for_each(|t| {
      let t1 = t[0] as usize;
      let t2 = t[1] as usize;
      dp[t1].0 += 1;
      dp[t2].1 += 1;
    });

    let mut idx: usize = 1;
    while idx < dp.len() {
      if dp[idx].0 == 0 && dp[idx].1 == n - 1 {
        return idx as i32;
      }
      idx += 1;
    }
    -1
  }
}
