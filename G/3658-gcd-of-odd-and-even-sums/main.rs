impl Solution {
  pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
    n
  }

  pub fn gcd_of_odd_even_sums2(n: i32) -> i32 {
    let mut odd: i32 = 0;
    let mut even: i32 = 0;
    for i in 1..=n * 2 {
      if i % 2 == 0 {
        even += i;
      } else {
        odd += i;
      }
    }

    fn gcd(x: i32, y: i32) -> i32 {
      let mut n1 = x;
      let mut n2 = y;
      if (x < y) {
        n1 = y;
        n2 = x;
      }

      let mut rem = n1 % n2;
      while (rem != 0) {
        n1 = n2;
        n2 = rem;
        rem = n1 % n2;
      }
      n2
    }

    gcd(odd, even)
  }
}
