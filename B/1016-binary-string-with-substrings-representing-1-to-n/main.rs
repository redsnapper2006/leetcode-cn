struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn query_string(s: String, n: i32) -> bool {
    let s_len: usize = s.len();
    let mut dp: Vec<[i32; 31]> = vec![[0; 31]; s_len];

    let bb = s.as_bytes().to_vec();
    let mut idx: usize = 0;
    while idx < bb.len() {
      let single: i32 = (bb[idx] - '0' as u8) as i32;
      dp[idx][0] = single;

      if idx > 0 {
        (1..31).for_each(|off| {
          dp[idx][off] = dp[idx - 1][off - 1] * 2 + single;
        });
      }
      idx += 1;
    }

    // println!("{:?}", dp);
    let mut res: HashSet<i32> = HashSet::new();
    dp.iter().for_each(|a| {
      (0..31).for_each(|idx| {
        if a[idx] <= n {
          res.insert(a[idx]);
        }
      });
    });

    let mut idx: usize = 1;
    while idx <= n as usize {
      if !res.contains(&(idx as i32)) {
        return false;
      }
      idx += 1;
    }

    true
  }
}

fn main() {
  println!("{}", Solution::query_string("0110".to_string(), 4));
  println!("{}", Solution::query_string("1111000101".to_string(), 5));

  println!("{}", Solution::query_string("011010101010111101010101011111111111111111111111111111111110000000000000011111101010101001010101010101010101010101111010101010111111111111111111111111111111111100000000000000111111010101010010101010101010101010100".to_string(), 1000000000));
}
