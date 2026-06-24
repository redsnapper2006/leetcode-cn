impl Solution {
  pub fn gcd_sum(nums: Vec<i32>) -> i64 {
    fn gcd(mut x: i64, mut y: i64) -> i64 {
      while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
      }
      x
    }

    let mut mx: i64 = nums[0] as i64;

    let mut gcds: Vec<i64> = nums
      .iter()
      .map(|&n| {
        mx = mx.max(n as i64);
        gcd(mx, n as i64)
      })
      .collect();
    gcds.sort_unstable();
    (0..gcds.len() / 2).fold(0, |ans, idx| {
      ans + gcd(gcds[idx], gcds[gcds.len() - 1 - idx])
    })
  }
}
