struct Solution {}

impl Solution {
  pub fn minimum_substrings_in_partition(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut dp: Vec<i32> = vec![-1; bb.len()];

    fn dfs(bb: &Vec<u8>, index: usize, dp: &mut Vec<i32>) -> i32 {
      if index == bb.len() {
        return 0;
      }
      if dp[index] != -1 {
        return dp[index];
      }

      let mut min: i32 = bb.len() as i32 + 1;

      let mut cnt: Vec<i32> = vec![0; 26];
      (index..bb.len()).for_each(|i| {
        cnt[(bb[i] - b'a') as usize] += 1;
        let mut c: i32 = 0;
        let mut valid: bool = true;
        let mut idx: usize = 0;
        while idx < 26 {
          if cnt[idx] > 0 {
            if c == 0 {
              c = cnt[idx];
            } else if c != cnt[idx] {
              valid = false;
              break;
            }
          }
          idx += 1;
        }
        if valid {
          let n = dfs(bb, i + 1, dp);
          if min > 1 + n {
            min = 1 + n;
          }
        }
      });
      dp[index] = min;
      min
    }

    dfs(&bb, 0, &mut dp)
  }
}

fn main() {
  println!(
    "{}",
    Solution::minimum_substrings_in_partition("fabccddg".to_string())
  );

  println!(
    "{}",
    Solution::minimum_substrings_in_partition("abababaccddb".to_string())
  );
  println!(
    "{}",
    Solution::minimum_substrings_in_partition("jwvjjjmmluuugmmmppdcccgpnmq".to_string())
  );
}
