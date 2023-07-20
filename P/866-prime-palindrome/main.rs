struct Solution {}

impl Solution {
  pub fn check_prime(nn: i32) -> bool {
    if nn < 2 {
      return false;
    }
    let stop = ((nn as f64).sqrt() + 1.0) as i32;
    for i in 2..stop {
      if nn % i == 0 {
        return false;
      }
    }
    true
  }

  pub fn reverse(n: i32) -> i32 {
    let mut nn = n;
    let mut c = 0;
    while nn > 0 {
      c = 10 * c + nn % 10;
      nn /= 10;
    }
    c
  }

  pub fn prime_palindrome(n: i32) -> i32 {
    let mut nn = n;
    while true {
      if nn == Self::reverse(nn) && Self::check_prime(nn) {
        return nn;
      }
      nn += 1;
      if nn > 10000000 && nn < 100000000 {
        nn = 100000000;
      }
    }
    0
  }
}

fn main() {
  println!("{}", Solution::prime_palindrome(1));
}
