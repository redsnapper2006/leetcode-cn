struct Solution {}

use std::cmp::Ordering;
impl Solution {
  pub fn rank_teams(votes: Vec<String>) -> String {
    if votes.len() == 1 {
      return votes[0].clone();
    }
    let mut buf: Vec<Vec<i32>> = vec![vec![0; 27]; 26];
    (0..26).for_each(|idx| {
      buf[idx][26] = b'A' as i32 + idx as i32;
    });

    votes.iter().for_each(|vote| {
      vote
        .as_bytes()
        .to_vec()
        .iter()
        .enumerate()
        .for_each(|(ii, b)| {
          buf[(b - b'A') as usize][ii] += 1;
        });
    });
    buf.sort_by(|x, y| {
      let mut idx: usize = 0;
      while idx < 26 {
        if x[idx] == y[idx] {
          idx += 1;
        } else {
          return y[idx].cmp(&x[idx]);
        }
      }
      x[26].cmp(&y[26])
    });

    let mut ans: Vec<u8> = Vec::new();
    buf.iter().for_each(|b| {
      if b[0..26].iter().any(|&x| x > 0) {
        ans.push(b[26] as u8);
      }
    });

    String::from_utf8(ans).unwrap()
  }
}
