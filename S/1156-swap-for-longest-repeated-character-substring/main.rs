struct Solution {}

impl Solution {
  pub fn max_rep_opt1(text: String) -> i32 {
    let mut dp: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 26];
    let bb: Vec<u8> = text.as_bytes().to_vec();
    let mut idx: usize = 0;

    let mut start: usize = idx;
    while idx < bb.len() {
      if bb[idx] != bb[start] {
        dp[(bb[start] - 'a' as u8) as usize].push((start, idx - 1));
        start = idx;
      }
      idx += 1;
    }
    dp[(bb[start] - 'a' as u8) as usize].push((start, idx - 1));
    *(0..26)
      .map(|idx| {
        if dp[idx].len() == 0 {
          return 0;
        }
        if dp[idx].len() == 1 {
          return dp[idx][0].1 - dp[idx][0].0 + 1;
        }
        let mut max: usize = 0;
        let mut iidx: usize = 0;
        let size = dp[idx].len();
        while iidx < size - 1 {
          let mut v = dp[idx][iidx].1 - dp[idx][iidx].0 + 1;
          if dp[idx][iidx + 1].0 - dp[idx][iidx].1 == 2 {
            v += dp[idx][iidx + 1].1 - dp[idx][iidx + 1].0 + 1;
            if size > 2 {
              v += 1;
            }
          } else {
            v += 1;
          }

          max = max.max(v);
          iidx += 1;
        }
        let mut v = dp[idx][size - 1].1 - dp[idx][size - 1].0 + 1;
        if size > 1 {
          v += 1;
        }
        max = max.max(v);

        max
      })
      .collect::<Vec<usize>>()
      .iter()
      .max()
      .unwrap() as i32
  }
}
