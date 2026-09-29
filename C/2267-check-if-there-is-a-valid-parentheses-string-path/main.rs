impl Solution {
  pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
    if grid[0][0] == ')' {
      return false;
    }

    let mut dp: Vec<(u128, u128)> = vec![(0, 0); grid[0].len()];
    dp[0] = (0, 2);

    let calc = |dp: (u128, u128), c: char| -> (u128, u128) {
      let mut ans = dp;
      if c == '(' {
        ans.0 <<= 1;
        if dp.1 & 0x80000000000000000000000000000000 > 0 {
          ans.0 += 1;
        }
        ans.1 <<= 1;
      } else {
        ans.0 >>= 1;
        ans.1 >>= 1;
      }
      ans
    };

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        if r == 0 && c == 0 {
          continue;
        }
        let mut up = (0u128, 0u128);
        if r > 0 {
          up = dp[c];
        }
        let mut left = (0u128, 0u128);
        if c > 0 {
          left = dp[c - 1];
        }
        dp[c] = calc((up.0 | left.0, up.1 | left.1), grid[r][c]);
      }
    }
    dp[grid[0].len() - 1].1 & 1 == 1
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::has_valid_path(vec![
      vec!['(', '(', '('],
      vec![')', '(', ')'],
      vec!['(', '(', ')'],
      vec!['(', '(', ')']
    ])
  );
}
