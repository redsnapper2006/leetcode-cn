impl Solution {
  pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;
    let mut nums = nums;
    nums.sort_unstable();
    let mut cnt: i64 = 0;
    let mut base: i64 = 0;

    let mut sum: i64 = 0;
    for nn in &nums {
      let n = *nn as i64;
      if n == base {
        continue;
      }

      let cc = (n - base - 1).min(k - cnt);
      sum += (base * 2 + 1 + cc) * cc / 2;
      cnt += cc;
      if cnt >= k {
        break;
      }

      base = *n;
    }

    if cnt < k as i64 {
      sum += (base * 2 + 1 + (k - cnt)) * (k - cnt) / 2;
    }

    sum
  }
}
