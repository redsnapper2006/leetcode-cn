struct Solution {}

impl Solution {
  pub fn pivot_integer(n: i32) -> i32 {
    let sum = n * (n + 1) / 2;
    let v = (sum as f64).sqrt();
    if v as i32 * v as i32 == sum {
      v as i32
    } else {
      -1
    }
  }

  pub fn pivot_integer2(n: i32) -> i32 {
    let (mut s, mut e): (i32, i32) = (1, n);

    while s <= e {
      let m = s + (e - s) / 2;
      let left = (1 + m) * m / 2;
      let right = (m + n) * (n - m + 1) / 2;
      if left == right {
        return m;
      } else if left > right {
        e = m - 1;
      } else {
        s = m + 1;
      }
    }
    -1
  }
}
