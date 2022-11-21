struct Solution {}

impl Solution {
  pub fn soup_servings(n: i32) -> f64 {
    let nn: i32 = (n + 24) / 25;
    if nn >= 197 {
      return 1.0;
    }
    let mut dp: Vec<Vec<f64>> = vec![vec![0.0; nn as usize + 1]; nn as usize + 1];
    dp[0][0] = 0.5;
    for i in 1..=nn {
      dp[0][i as usize] = 1.0;
    }

    fn max(a: i32, b: i32) -> usize {
      if a > b {
        return a as usize;
      }
      b as usize
    }
    for i in 1..=nn {
      for j in 1..=nn {
        dp[i as usize][j as usize] = (dp[max(0, i - 4)][j as usize]
          + dp[max(0, i - 3)][j as usize - 1]
          + dp[max(0, i - 2)][max(0, j - 2)]
          + dp[i as usize - 1][max(0, j - 3)])
          * 0.25;
      }
    }
    dp[nn as usize][nn as usize]
  }
}

fn main() {
  println!("{}", Solution::soup_servings(850));
}
