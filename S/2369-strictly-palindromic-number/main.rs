struct Solution {}

impl Solution {
  pub fn is_strictly_palindromic(n: i32) -> bool {
    false
  }

  pub fn is_strictly_palindromic2(n: i32) -> bool {
    for i in 2..=n - 2 {
      let mut b: Vec<i32> = Vec::new();
      let mut nn: i32 = n;
      while nn > 0 {
        b.push(nn % i);
        nn /= i;
      }
      for j in 0..b.len() / 2 {
        if b[j] != b[b.len() - 1 - j] {
          return false;
        }
      }
    }
    true
  }
}
