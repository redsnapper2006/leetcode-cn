impl Solution {
  pub fn sum_and_multiply(n: i32) -> i64 {
    let mut n = n as i64;
    let (mut sum, mut aggr, mut times): (i64, i64, i64) = (0, 0, 1);
    while n > 0 {
      let nn = n % 10;
      sum += nn;
      if nn != 0 {
        aggr += times * nn;
        times *= 10;
      }
      n /= 10;
    }
    aggr * sum
  }
}
