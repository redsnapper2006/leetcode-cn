impl Solution {
  pub fn check_good_integer(n: i32) -> bool {
    let mut sum: i32 = 0;
    let mut prod: i32 = 0;
    let mut n = n;
    while n > 0 {
      let nn = n % 10;

      n /= 10;
      sum += nn;
      prod += nn * nn;
    }

    (prod - sum).abs() >= 50
  }
}
