impl Solution {
  pub fn three_fibonacci_sum(n: i32) -> bool {
    let (mut a1, mut a2, mut a3) = (0i32, 1i32, 1i32);

    while a1 + a2 + a3 < n {
      (a1, a2, a3) = (a2, a3, a2 + a3);
    }
    a1 + a2 + a3 == n
  }

  pub fn three_fibonacci_sum2(n: i32) -> bool {
    if n & 1 == 1 {
      return false;
    }

    let half = n >> 1;
    let (mut a, mut b) = (0i32, 1i32);
    while b < half {
      (a, b) = (b, a + b);
    }
    b == half
  }
}
