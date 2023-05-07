struct Solution {}

impl Solution {
  pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut buf: [i64; 60] = [0; 60];

    time.iter().for_each(|&v| {
      buf[(v % 60) as usize] += 1;
    });

    let mut res: i64 = 0;
    (1..30).for_each(|v| {
      res += buf[v] * (buf[60 - v]);
    });
    (res + buf[0] * (buf[0] - 1) / 2 + buf[30] * (buf[30] - 1) / 2) as i32
  }
}
