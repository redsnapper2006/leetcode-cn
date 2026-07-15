impl Solution {
  pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
  }

  pub fn gcd_of_odd_even_sums2(n: i32) -> i32 {
    let n = n as i64;
    let mut odd: i64 = n*(n+1);
    let mut even: i64 = n*n;

    let gcd = |mut x :i64, mut y : i64| -> i32 {
      while y != 0 {
        let t = y;
        y = x % y;
        x = t;
      }
      x as i32
    };

    gcd(odd, even)
  }
}
