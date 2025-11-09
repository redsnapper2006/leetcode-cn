impl Solution {
  pub fn count_operations(num1: i32, num2: i32) -> i32 {
    let mut num1 = num1;
    let mut num2 = num2;
    let mut ans: i32 = 0;
    unsafe {
      while num1 != 0 && num2 != 0 {
        let p1 = if num1 > num2 {
          &raw mut num1
        } else {
          &raw mut num2
        };
        let p2 = if num1 > num2 {
          &raw mut num2
        } else {
          &raw mut num1
        };
        ans += *p1 / *p2;
        *p1 %= *p2;
      }
    }
    ans
  }
}
