struct Solution {}

impl Solution {
  pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![1; n as usize];
    let mut idx: Vec<i32> = vec![0; primes.len()];
    let max: i32 = 0x7FFFFFFF;
    for i in 1..n {
      let mut min = 0x7FFFFFFF;
      for j in 0..idx.len() {
        if buf[idx[j] as usize] * primes[j] < min {
          min = buf[idx[j] as usize] * primes[j];
        }
      }
      buf[i as usize] = min;
      for j in 0..idx.len() {
        if buf[idx[j] as usize] * primes[j] == min {
          idx[j] += 1;
        }
      }
    }
    buf[n as usize - 1]
  }
}

fn main() {
  println!("{}", Solution::nth_super_ugly_number(5, vec![2, 7, 13, 19]));
  println!(
    "{}",
    Solution::nth_super_ugly_number(
      100000,
      vec![
        7, 19, 29, 37, 41, 47, 53, 59, 61, 79, 83, 89, 101, 103, 109, 127, 131, 137, 139, 157, 167,
        179, 181, 199, 211, 229, 233, 239, 241, 251
      ]
    )
  );
}
