struct Solution {}

impl Solution {
  pub fn check_record(n: i32) -> i32 {
    // a0l0, a0l1, a0l2, a1l0, a1l1, a1l2
    let mut dp: (i64, i64, i64, i64, i64, i64) = (1, 0, 0, 0, 0, 0);
    let MOD = 1000000007 as i64;
    (0..n).for_each(|_| {
      dp = (
        (dp.0 + dp.1 + dp.2) % MOD,
        dp.0,
        dp.1,
        (dp.0 + dp.1 + dp.2 + dp.3 + dp.4 + dp.5) % MOD,
        dp.3,
        dp.4,
      );
    });
    ((dp.0 + dp.1 + dp.2 + dp.3 + dp.4 + dp.5) % MOD) as i32
  }
}
