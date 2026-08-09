impl Solution {
  pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
    let n = piles.len();
    let mut suffix = vec![0i32; n + 1];
    for i in (0..n).rev() {
      suffix[i] = suffix[i + 1] + piles[i];
    }

    let mut memo = vec![vec![-1; n + 1]; n + 1];
    fn dfs(s_idx: usize, m: usize, suffix: &[i32], memo: &mut [Vec<i32>], n: usize) -> i32 {
      if s_idx >= n {
        return 0;
      }
      if memo[s_idx][m] != -1 {
        return memo[s_idx][m];
      }

      let mut max_accum: i32 = 0;
      for idx in 1..2*m{
        if s_idx + idx > n {
          break;
        }

        let accum = dfs(s_idx + idx, idx.max(m), suffix, memo, n);
        max_accum = max_accum.max(suffix[s_idx] - accum);
      });
      memo[s_idx][m] = max_accum;
      max_accum
    }

    dfs(0, 1, &suffix, &mut memo, n)
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::stone_game_ii(vec![
      8270, 7145, 575, 5156, 5126, 2905, 8793, 7817, 5532, 5726, 7071, 7730, 5200, 5369, 5763,
      7148, 8287, 9449, 7567, 4850, 1385, 2135, 1737, 9511, 8065, 7063, 8023, 7729, 7084, 8407
    ])
  )
}
