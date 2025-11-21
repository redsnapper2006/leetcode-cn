impl Solution {
  pub fn count_palindromic_subsequence(s: String) -> i32 {
    let mut pos: Vec<(usize, usize, i32)> = vec![(0, 0, 0); 26];
    let bb = s.as_bytes().to_vec();
    for i in (0..bb.len()) {
      let off = (bb[i] - b'a') as usize;
      if pos[off].2 == 0 {
        pos[off] = (i, i, 1);
      } else {
        pos[off].1 = i;
        pos[off].2 += 1;
      }
    }
    let mut dp: Vec<Vec<i32>> = vec![];
    let mut sum: Vec<i32> = vec![0; 26];
    for i in (0..bb.len()) {
      let off = (bb[i] - b'a') as usize;
      sum[off] += 1;
      dp.push(sum.clone());
    }

    let mut ans: i32 = 0;
    for i in 0..26 {
      if pos[i].2 < 2 || pos[i].1 == pos[i].0 + 1 {
        continue;
      }
      for j in 0..26 {
        if dp[pos[i].1 - 1][j] - dp[pos[i].0][j] > 0 {
          ans += 1;
        }
      }
    }
    ans
  }
}
