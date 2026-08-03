impl Solution {
  pub fn count_visible_people(n: i32, pos: i32, k: i32) -> i32 {
    let (n, k): (i64, i64) = (n as i64, k as i64);
    const MOD: i64 = 1000000007i64;
    let mut fac: i64 = 1;
    for i in 1..n {
      fac = fac * i % MOD;
    }
    let pow = |mut x: i64, mut n: i64| -> i64 {
      let mut ans: i64 = 1;
      while n > 0 {
        if n & 1 == 1 {
          ans = ans * x % MOD;
        }
        x = x * x % MOD;
        n >>= 1;
      }
      ans
    };
    let mut invF: Vec<i64> = vec![0; n as usize];
    invF[n as usize - 1] = pow(fac, MOD - 2);
    for i in (1..n).rev() {
      invF[i as usize - 1] = invF[i as usize] * i % MOD;
    }

    ((fac * invF[k as usize] % MOD * invF[(n - 1 - k) as usize] % MOD) * 2 % MOD) as i32
  }
}
