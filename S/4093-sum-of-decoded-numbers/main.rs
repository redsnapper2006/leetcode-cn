impl Solution {
  pub fn sum_decoded(nums: Vec<i64>) -> i32 {
    let MOD = 1000000007;
    let mut ans = 0i64;

    nums.iter().for_each(|&n| {
      let width = n % 10;
      let mut d = n / 10;
      let mut digits: Vec<i64> = vec![];
      while d > 0 {
        digits.push(d % 10);
        d /= 10;
      }

      let mut y = 0i64;
      for i in (0..(digits.len() - width as usize)).rev() {
        y = y * 10 + digits[i];
      }
      let mut x = 0i64;
      for i in ((digits.len() - width as usize)..digits.len()).rev() {
        x = x * 10 + digits[i];
      }
      x %= MOD;

      let POW = |x: i64, n: i64| -> i64 {
        let mut res: i64 = 1;
        let mut n = n;
        let mut x = x;
        while n > 0 {
          if n % 2 > 0 {
            res = res * x % MOD;
          }
          x = x * x % MOD;
          n /= 2;
        }
        res
      };
      ans += POW(x, y);

      ans %= MOD;
    });
    ans as i32
  }
}
