impl Solution {
  pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    digits.sort_unstable();
    let mut ans: Vec<i32> = vec![];
    let mut h: i32 = -1;
    let mut t: i32;
    let mut d: i32;
    for i in 0..digits.len() {
      if digits[i] == 0 || i > 0 && digits[i] == h {
        continue;
      }
      h = digits[i];
      t = -1;
      for j in 0..digits.len() {
        if i == j || j > 0 && digits[j] == t {
          continue;
        }
        t = digits[j];
        d = -1;
        for k in 0..digits.len() {
          if i == k || j == k || digits[k] % 2 == 1 || k > 0 && digits[k] == d {
            continue;
          }
          d = digits[k];
          ans.push(digits[i] * 100 + digits[j] * 10 + digits[k]);
        }
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{:?}", Solution::find_even_numbers(vec![1, 2, 3, 0]));
}
