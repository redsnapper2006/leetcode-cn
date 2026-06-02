impl Solution {
  pub fn digit_frequency_score(n: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; 10];
    let mut n = n;
    while n > 0 {
      buf[n as usize % 10] += 1;
      n /= 10;
    }
    buf.iter().enumerate().fold(0, |ans, (idx, c)| ans + idx as i32 * c)
  }
}
