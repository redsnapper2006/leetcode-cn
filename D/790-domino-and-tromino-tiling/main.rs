struct Solution {}

impl Solution {
  pub fn num_tilings(n: i32) -> i32 {
    let mode: i32 = 1000000007;
    let mut dp: Vec<[i32; 4]> = vec![[0, 0, 0, 1]];
    for i in 1..=n {
      let v: [i32; 4] = [
        dp[(i as usize - 1)][3],
        (dp[i as usize - 1][0] + dp[i as usize - 1][2]) % mode,
        (dp[i as usize - 1][0] + dp[i as usize - 1][1]) % mode,
        (((dp[i as usize - 1][0] + dp[i as usize - 1][1]) % mode + dp[i as usize - 1][2]) % mode
          + dp[i as usize - 1][3])
          % mode,
      ];
      dp.push(v);
    }
    dp[n as usize][3]
  }
}

fn main() {
  println!("{}", Solution::num_tilings(5));
}
