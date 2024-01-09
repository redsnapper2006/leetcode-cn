struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let mut dict: HashSet<String> = HashSet::new();
    dictionary.iter().for_each(|v| {
      dict.insert(v.to_string());
    });

    let bb: Vec<u8> = s.bytes().collect();
    let mut dp: Vec<i32> = vec![-1; bb.len()];
    fn dfs(bb: &Vec<u8>, idx: usize, dict: &HashSet<String>, dp: &mut Vec<i32>) -> i32 {
      if idx == bb.len() {
        return 0;
      }

      if dp[idx] != -1 {
        return dp[idx];
      }

      let mut idx2 = idx + 1;
      let mut res: i32 = i32::MAX;
      while idx2 <= bb.len() {
        let next = String::from_utf8(bb[idx..idx2].to_vec()).unwrap();

        let mut remain = dfs(bb, idx2, dict, dp);

        if !dict.contains(&next) {
          remain += (idx2 - idx) as i32;
        }
        if remain < res {
          res = remain;
        }
        idx2 += 1;
      }
      dp[idx] = res;
      res
    }
    dfs(&bb, 0, &dict, &mut dp);

    dp[0]
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_extra_char(
      "leetscode".to_string(),
      vec![
        "leet".to_string(),
        "code".to_string(),
        "leetcode".to_string()
      ]
    )
  );
}
