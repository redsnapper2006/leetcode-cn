struct Solution {}

impl Solution {
  pub fn count_vowel_strings(n: i32) -> i32 {
    let mut buf: Vec<[i32; 5]> = vec![[1; 5]; n as usize];

    (1..n).for_each(|i| {
      let mut sum: i32 = 0;
      (0..5).rev().for_each(|n| {
        sum += buf[i as usize - 1][n];
        buf[i as usize][n] = sum;
      });
    });

    buf[n as usize - 1][0]
      + buf[n as usize - 1][1]
      + buf[n as usize - 1][2]
      + buf[n as usize - 1][3]
      + buf[n as usize - 1][4]
  }
}
