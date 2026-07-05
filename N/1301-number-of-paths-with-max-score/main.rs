impl Solution {
  pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
    const MOD: i64 = 1000000007;
    let bb = board.iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let update = |r: usize, c: usize, nr: usize, nc: usize, dp: &mut Vec<Vec<(i64, i64)>>| {
      if nr == bb.len() || nc == bb[0].len() || dp[nr][nc].1 <= 0 {
        return;
      }

      if dp[nr][nc].0 > dp[r][c].0 {
        dp[r][c] = dp[nr][nc];
      } else if dp[nr][nc].0 == dp[r][c].0 {
        dp[r][c].1 += dp[nr][nc].1;
        dp[r][c].1 %= MOD;
      }
    };

    let mut dp: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0); bb[0].len()]; bb.len()];
    for r in (0..bb.len()).rev() {
      for c in (0..bb[0].len()).rev() {
        if r == bb.len() - 1 && c == bb[0].len() - 1 {
          dp[r][c] = (0, 1);
          continue;
        }
        if bb[r][c] == b'X' {
          continue;
        }

        update(r, c, r, c + 1, &mut dp);
        update(r, c, r + 1, c, &mut dp);
        update(r, c, r + 1, c + 1, &mut dp);

        if dp[r][c].1 > 0 {
          dp[r][c].0 += if r == 0 && c == 0 {
            0
          } else {
            (bb[r][c] - b'0') as i64
          };
        }
      }
    }

    vec![
      (dp[0][0].0 % 1000000007) as i32,
      (dp[0][0].1 % 1000000007) as i32,
    ]
  }
}
