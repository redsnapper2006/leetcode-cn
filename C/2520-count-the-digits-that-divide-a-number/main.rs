struct Solution {}

impl Solution {
  pub fn count_digits(num: i32) -> i32 {
    let mut b: Vec<i32> = Vec::new();
    let mut n = num;

    while n > 0 {
      b.push(n % 10);
      n /= 10;
    }
    let mut bb: [i32; 10] = [0; 10];
    b.iter().for_each(|&v| {
      if num % v == 0 {
        bb[v as usize] += 1;
      }
    });
    bb.iter().sum()
  }
}
