impl Solution {
  pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut sum = 0i32;
    let mut ans = 0i32;
    for b in s.bytes() {
      sum += if b == b'(' { 1 } else { -1 };
      if sum < 0 {
        ans += 1;
        sum = 0;
      }
    }
    ans + sum
  }
}
