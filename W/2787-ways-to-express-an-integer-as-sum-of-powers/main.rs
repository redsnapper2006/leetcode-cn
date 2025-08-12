impl Solution {
  pub fn number_of_ways(n: i32, x: i32) -> i32 {
    let _m: usize = 1000000007;
    let mut n = n as usize;
    let mut keys: Vec<usize> = vec![];
    for i in 1..=n {
      let mut p: usize = 1;
      for _ in 0..x {
        p *= i;
      }
      if p > n {
        break;
      }
      keys.push(p);
    }

    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 1;
    for k in keys.iter() {
      let mut t: Vec<usize> = vec![0; n + 1];
      for j in 0..n + 1 {
        if dp[j] == 0 || j + k > n {
          continue;
        }
        t[j + k] += dp[j];
        t[j + k] %= _m;
      }
      for j in 0..n + 1 {
        dp[j] += t[j];
        dp[j] %= _m;
      }
    }

    dp[n] as _
  }
}
