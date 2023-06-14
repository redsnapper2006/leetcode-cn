struct Solution {}

impl Solution {
  pub fn is_fascinating(n: i32) -> bool {
    let mut d: Vec<i32> = vec![0; 10];
    fn digits(n: i32, d: &mut Vec<i32>) {
      let mut m = n;
      while m > 0 {
        d[(m % 10) as usize] += 1;
        m /= 10;
      }
    }
    digits(n, &mut d);
    digits(n * 2, &mut d);
    digits(n * 3, &mut d);

    d[1..].iter().all(|&v| v == 1) && d[0] == 0
  }
}
