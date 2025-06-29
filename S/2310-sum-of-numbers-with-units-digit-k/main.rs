impl Solution {
  pub fn minimum_numbers(num: i32, k: i32) -> i32 {
    if num == 0 {
      return 0;
    }
    if k == 0 {
      return if num % 10 == 0 { 1 } else { -1 };
    }
    let mut num = num;
    let mut cnt: i32 = 0;
    while num >= k {
      num -= k;
      cnt += 1;
      if num % 10 == 0 {
        return cnt;
      }
    }

    -1
  }
}
