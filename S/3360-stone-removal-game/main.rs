impl Solution {
  pub fn can_alice_win(n: i32) -> bool {
    let m: i64 = 441i64 - 8 * n as i64;

    let mut x = m;
    let mut y = (x + 1) / 2;
    while y < x {
      x = y;
      y = (x + m / x) / 2;
    }

    let mut k = (21 - x) / 2;
    while (21 * k - k * k) / 2 > n as i64 {
      k -= 1;
    }

    k % 2 == 1
  }
}
