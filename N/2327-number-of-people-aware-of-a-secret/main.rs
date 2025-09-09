impl Solution {
  pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    let M: i64 = 1000000007;
    let n = n as usize;
    let delay = delay as usize;
    let forget = forget as usize;
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 3]; n + 1 + delay];
    dp[1][0] = 1;
    for i in 2..=n {
      dp[i][1] = (if i < delay { 0 } else { dp[i - delay][0] } + dp[i - 1][1]
        - if i < forget { 0 } else { dp[i - forget][0] }
        + M)
        % M;
      dp[i][0] = dp[i][1] % M;
    }

    for i in n + 1..n + 1 + delay {
      dp[i][1] = (if i < delay { 0 } else { dp[i - delay][0] } + dp[i - 1][1]) % M;
      dp[i][0] = dp[i][1] % M;
    }

    dp[n + delay][1] as _
  }
}

struct Solution {}

fn main() {
  println!("{:?}", Solution::people_aware_of_secret(6, 2, 4));
  println!("{:?}", Solution::people_aware_of_secret(4, 1, 3));
}
