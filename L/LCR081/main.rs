struct Solution {}

impl Solution {
  pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort_unstable();
    let mut dp: Vec<Vec<Vec<i32>>> = vec![Vec::new(); target as usize + 1];
    dp[0] = vec![Vec::new()];
    (0..target as usize).for_each(|idx| {
      if dp[idx].len() == 0 {
        return;
      }
      candidates.iter().for_each(|&candi| {
        if candi + idx as i32 > target {
          return;
        }
        dp[idx].clone().into_iter().for_each(|next| {
          if next.len() > 0 && next[next.len()-1] < candi {
            return;
          }
          let mut n = next.clone();
          n.push(candi);
          dp[candi as usize + idx].push(n);
        });
      });
    });
    dp[target as usize].clone()
  }
}
