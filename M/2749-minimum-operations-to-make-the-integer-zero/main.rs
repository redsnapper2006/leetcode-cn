impl Solution {
  pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    let mut num1 = num1 as i64;
    let num2 = num2 as i64;
    let mut times: u32 = 0;
    while num1 > times as i64 {
      num1 -= num2;
      times += 1;
      if num1 >= times as i64 && num1.count_ones() <= times {
        return times as _;
      }
    }
    -1
  }
}
