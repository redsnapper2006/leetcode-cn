impl Solution {
  pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
    let mut n1 = num1;
    let mut n2 = num2;
    let mut n3 = num3;

    let mut ans: Vec<i32> = Vec::new();
    while n1 > 0 || n2 > 0 || n3 > 0 {
      ans.push((n1 % 10).min(n2 % 10).min(n3 % 10));
      n1 /= 10;
      n2 /= 10;
      n3 /= 10;
    }
    let mut sum: i32 = 0;
    ans.iter().rev().for_each(|&v| {
      sum = sum * 10 + v;
    });
    sum
  }
}
