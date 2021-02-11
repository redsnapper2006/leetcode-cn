struct Solution {}

impl Solution {
  pub fn gcd(m: i32, n: i32) -> i32 {
    assert!(m != 0 && n != 0);
    if m > n {
      return Solution::gcd(m - n, n);
    };
    if m < n {
      return Solution::gcd(n - m, m);
    };
    m
  }

  pub fn simplified_fractions(n: i32) -> Vec<String> {
    let mut buf: Vec<String> = Vec::new();

    for i in 2..n + 1 {
      buf.push(format!("{}/{}", 1, i));
      for j in 2..i {
        if Solution::gcd(i, j) > 1 {
          continue;
        }
        buf.push(format!("{}/{}", j, i));
      }
    }
    buf
  }
}

fn main() {
  println!("{}", Solution::simplified_fractions(3));
}
