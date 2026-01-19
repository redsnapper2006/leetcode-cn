impl Solution {
  pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; mat[0].len()]; mat.len()];
    for i in 0..mat.len() {
      for j in 0..mat[0].len() {
        buf[i][j] = if j > 0 { buf[i][j - 1] } else { 0 } + if i > 0 { buf[i - 1][j] } else { 0 }
          - if i > 0 && j > 0 { buf[i - 1][j - 1] } else { 0 }
          + mat[i][j];
      }
    }

    let mut ans: i32 = 0;
    for i in 0..mat.len() {
      for j in 0..mat[0].len() {
        let w = (i + 1).min(j + 1);
        for n in (ans as usize + 1)..=w {
          if buf[i][j]
            - if i > n { buf[i - n][j] } else { 0 }
            - if j > n { buf[i][j - n] } else { 0 }
            + if i > n && j > n { buf[i - n][j - n] } else { 0 }
            <= threshold
          {
            ans = ans.max(n as i32);
          }
        }
      }
    }
    ans
  }
}
