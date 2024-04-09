impl Solution {
  pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut xx = x;
    while xx > 0 {
      sum += xx % 10;
      xx /= 10;
    }
    if x % sum == 0 {
      sum
    } else {
      -1
    }
  }
}
