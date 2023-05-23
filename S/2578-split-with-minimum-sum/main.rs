struct Solution {}

impl Solution {
  pub fn split_num(num: i32) -> i32 {
    let mut n = num;
    let mut b: Vec<i32> = Vec::new();

    while n > 0 {
      b.push(n % 10);
      n /= 10;
    }
    b.sort();
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut idx: usize = 0;
    while idx < b.len() {
      n1 = n1 * 10 + b[idx];
      idx += 1;
      if idx < b.len() {
        n2 = n2 * 10 + b[idx];
      }
      idx += 1;
    }
    n1 + n2
  }
}
