impl Solution {
  pub fn total_waviness(num1: i32, num2: i32) -> i32 {
    fn digits(num: i32, digit: &mut Vec<i32>) {
      let mut n = num;
      while n > 0 {
        digit.push(n % 10);
        n /= 10;
      }
    }
    let mut digit1: Vec<i32> = vec![];
    let mut digit2: Vec<i32> = vec![];
    digits(num1, &mut digit1);
    digits(num2 + 1, &mut digit2);
    fn cmp(digit1: &Vec<i32>, digit2: &Vec<i32>) -> bool {
      if digit1.len() < digit2.len() {
        return true;
      }
      for i in (0..digit1.len()).rev() {
        if digit1[i] < digit2[i] {
          return true;
        }
      }
      false
    }

    let mut ans: i32 = 0;
    while cmp(&digit1, &digit2) {
      for i in 1..digit1.len() - 1 {
        if digit1[i] > digit1[i - 1] && digit1[i] > digit1[i + 1]
          || digit1[i] < digit1[i - 1] && digit1[i] < digit1[i + 1]
        {
          ans += 1;
        }
      }
      let mut stop: bool = false;
      for i in 0..digit1.len() {
        if digit1[i] < 9 {
          digit1[i] += 1;
          stop = true;
          break;
        }
        digit1[i] = 0;
      }
      if !stop {
        digit1.push(1);
      }
    }
    ans
  }
}
