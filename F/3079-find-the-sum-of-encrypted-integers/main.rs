impl Solution {
  pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |sum, &num| {
      let mut num = num;
      let mut digits: i32 = 0;
      let mut max: i32 = 0;
      while num > 0 {
        let digit = num % 10;
        if digit > max {
          max = digit;
        }
        digits += 1;
        num /= 10;
      }
      let mut num: i32 = 0;
      while digits > 0 {
        num *= 10;
        num += max;
        digits -= 1;
      }
      sum + num
    })
  }
}
