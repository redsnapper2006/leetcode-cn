impl Solution {
  pub fn number_of_sets(n: i32, k: i32) -> i32 {
    const M: i64 = 1000000007;
    let n = n as usize;
    let k = k as usize;
    let mut dp: Vec<Vec<(i64, i64)>> = vec![vec![(0, 0); k + 1]; n];

    for ii in 1..n {
      dp[ii][0] = (1, 1);
      dp[ii][1] = ((dp[ii - 1][1].0 + dp[ii - 1][1].1) % M, ii as i64);
      for kk in 2..=k {
        dp[ii][kk] = (
          (dp[ii - 1][kk].0 + dp[ii - 1][kk].1) % M,
          (dp[ii - 1][kk - 1].0 + dp[ii - 1][kk - 1].1 + dp[ii - 1][kk].1) % M,
        );
      }
    }

    let v = *dp.last().unwrap().last().unwrap();
    ((v.0 + v.1) % M) as i32
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::number_of_sets(4, 2));
  println!("{}", Solution::number_of_sets(3, 1));
  println!("{}", Solution::number_of_sets(5, 3));
  println!("{}", Solution::number_of_sets(30, 7));
}
