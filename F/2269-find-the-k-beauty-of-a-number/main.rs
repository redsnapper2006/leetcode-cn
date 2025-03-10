struct Solution {}

impl Solution {
  pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let mut base: i32 = 1;
    (0..k).for_each(|_| {
      base *= 10;
    });

    let mut n = num;
    let mut ans: i32 = 0;
    while n >= base / 10 {
      ans += if n % base != 0 && num % (n % base) == 0 {
        1
      } else {
        0
      };
      n /= 10;
    }
    ans
  }
}
