impl Solution {
  pub fn min_anagram_length(s: String) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; 26]; s.len() + 1];

    s.as_bytes().iter().enumerate().for_each(|(idx, b)| {
      (0..26).for_each(|ii| {
        dp[idx + 1][ii] = dp[idx][ii];
      });
      dp[idx + 1][(b - b'a') as usize] += 1;
    });

    let ll = s.len() as i32;
    let mut base: i32 = 1;
    while base <= ll / 2 {
      if ll % base != 0 {
        base += 1;
        continue;
      }

      let mut iii: i32 = base + base;

      let mut step_valid: bool = true;
      while iii <= s.len() as i32 {
        let mut valid: bool = true;
        let mut i: usize = 0;
        while i < 26 {
          if dp[base as usize][i] - dp[0][i] != dp[iii as usize][i] - dp[(iii - base) as usize][i] {
            valid = false;
            break;
          }
          i += 1;
        }
        if !valid {
          step_valid = false;
          break;
        }

        iii += base;
      }

      if step_valid {
        return base;
      }
      base += 1;
    }

    ll
  }
}
