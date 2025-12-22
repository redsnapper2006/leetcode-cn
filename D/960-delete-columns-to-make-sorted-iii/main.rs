impl Solution {
  pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let bbs = strs.iter().map(|str| str.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut dp: Vec<i32> = vec![1; strs[0].len()];

    for c in (0..bbs[0].len() - 1).rev() {
      let mut cnt: i32 = 1;
      for cc in c + 1..bbs[0].len() {
        let mut valid: bool = true;
        for r in 0..bbs.len() {
          if bbs[r][c] > bbs[r][cc] {
            valid = false;
            break;
          }
        }
        cnt = if valid { cnt.max(dp[cc] + 1) } else { cnt };
      }
      dp[c] = cnt;
    }

    strs[0].len() as i32 - *dp.iter().max().unwrap()
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::min_deletion_size(vec!["babca".to_string(), "bbazb".to_string()])
  );
  println!("{}", Solution::min_deletion_size(vec!["edcba".to_string()]));
  println!(
    "{}",
    Solution::min_deletion_size(vec![
      "ghi".to_string(),
      "def".to_string(),
      "abc".to_string()
    ])
  );
  println!(
    "{}",
    Solution::min_deletion_size(vec![
      "bbbbaaa".to_string(),
      "babbaaa".to_string(),
      "bbbbaab".to_string(),
      "abababb".to_string()
    ])
  );
}
