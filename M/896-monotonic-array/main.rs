struct Solution {}

impl Solution {
  pub fn is_monotonic(a: Vec<i32>) -> bool {
    if a.len() <= 2 {
      return true;
    }
    let mut isFirst = true;
    let mut isAsc = true;
    for i in 1..a.len() {
      if isFirst {
        if a[i] > a[i - 1] {
          isAsc = true;
          isFirst = false;
        } else if a[i] < a[i - 1] {
          isAsc = false;
          isFirst = false;
        }
      } else {
        if a[i] == a[i - 1] {
          continue;
        }
        if isAsc != (a[i] > a[i - 1]) {
          return false;
        }
      }
    }
    true
  }
}

fn main() {
  println!("{}", Solution::is_monotonic(vec![0; 5]));
}
