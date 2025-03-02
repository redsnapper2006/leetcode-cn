struct Solution {}

impl Solution {
  pub fn min_cut(s: String) -> i32 {
    fn is_pal(bb: &Vec<u8>, start: usize, end: usize) -> bool {
      let mut s = start as i32;
      let mut e = end as i32;
      while s <= e {
        if bb[s as usize] != bb[e as usize] {
          return false;
        }
        s += 1;
        e -= 1;
      }
      true
    }
    let bb = s.as_bytes().to_vec();
    let mut dp: Vec<i32> = vec![0; bb.len()];
    fn dfs(bb: &Vec<u8>, idx: usize, dp: &mut Vec<i32>) -> i32 {
      if (idx == bb.len()) {
        return 0;
      }

      if dp[idx] != 0 {
        return dp[idx];
      }

      let mut ans: i32 = (bb.len() - idx) as i32;
      (idx..bb.len()).for_each(|end| {
        if is_pal(bb, idx, end) {
          ans = ans.min(dfs(bb, end + 1, dp) + 1);
        }
      });
      dp[idx] = ans;
      ans
    }
    dfs(&bb, 0, &mut dp);
    dp[0] - 1
  }
}
