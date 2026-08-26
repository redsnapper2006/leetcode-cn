impl Solution {
  pub fn max_pair_strength(nums: Vec<i32>) -> i64 {
    let gcd = |mut x: i64, mut y: i64| -> i64 {
      while y != 0 {
        let t = y;
        y = x % y;
        x = t;
      }
      x
    };

    let mut nums = nums;
    nums.sort_unstable();
    let mut all_gcd: i64 = 0;
    for &n in &nums {
      all_gcd = gcd(all_gcd, n as i64);
    }
    let all_gcd2 = all_gcd * all_gcd;

    let mut ans = 0i64;
    for i in (0..nums.len()).rev() {
      let n1 = nums[i] as i64;
      for j in (i + 1..nums.len()).rev() {
        let n2 = nums[j] as i64;
        if n1 * n2 / all_gcd2 <= ans {
          break;
        }
        let g = gcd(n1, n2);
        ans = ans.max(n1 * n2 / g / g);
      }
    }
    ans
  }
}
