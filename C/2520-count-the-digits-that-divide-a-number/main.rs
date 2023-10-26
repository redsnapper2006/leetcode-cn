struct Solution {}

impl Solution {
  pub fn count_digits(num: i32) -> i32 {
    let mut b: Vec<i32> = Vec::new();
    let mut n = num;
    let mut cnt: i32 = 0;
    while n > 0 {
      if num % (n % 10) == 0 {
        cnt += 1;
      }

      n /= 10;
    }
    cnt
  }
}
