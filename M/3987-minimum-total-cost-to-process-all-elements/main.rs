impl Solution {
  pub fn minimum_cost(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as i64;
    const MOD: i64 = 1000000007;
    let mut kk = k;
    let mut base: i64 = 1;
    let mut ans: i64 = 0;
    for n in nums {
      let n = n as i64;
      if kk < n {
        let t = (n - kk + k - 1) / k;
        kk += t * k;
        ans = (ans + ((base + base + t - 1) * t) / 2) % MOD;
        base = (base + t) % MOD;
      }
      kk -= n;
    }
    ans as _
  }
}
