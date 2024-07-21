impl Solution {
  pub fn count_complete_day_pairs(hours: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![0; 24];
    hours.iter().for_each(|&v| {
      buf[(v % 24) as usize] += 1;
    });

    let mut ans: i32 = buf[0] * (buf[0] - 1) / 2 + buf[12] * (buf[12] - 1) / 2;
    (1..12).for_each(|idx| {
      ans += buf[idx] * buf[24 - idx];
    });
    ans
  }
}
