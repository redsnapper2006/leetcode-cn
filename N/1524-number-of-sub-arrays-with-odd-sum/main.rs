impl Solution {
  pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let (mut odd, mut even) = (0i64, 1i64);
    let mut parity = 0;
    let mut ans = 0i64;
    for &n in &arr {
      parity ^= n & 1;
      if parity == 1 {
        ans += even;
        odd += 1;
      } else {
        ans += odd;
        even += 1;
      }
      ans %= MOD;
    }
    ans as i32
  }
}
