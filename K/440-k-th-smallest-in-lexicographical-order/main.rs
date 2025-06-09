impl Solution {
  pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut k = k as i64 - 1;
    let n = n as i64;
    let mut cur: i64 = 1;

    fn steps(cur: i64, n: i64) -> i64 {
      let (mut first, mut last) = (cur, cur);
      let mut sum: i64 = 0;
      while first <= n {
        sum += n.min(last) - first + 1;
        first *= 10;
        last *= 10;
        last += 9;
      }

      sum
    }

    while k > 0 {
      let sum = steps(cur, n);
      if sum <= k {
        k -= sum;
        cur += 1;
      } else {
        cur *= 10;
        k -= 1;
      }
    }
    cur as _
  }
}
