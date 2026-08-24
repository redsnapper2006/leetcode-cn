impl Solution {
  pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
    let n = num as usize;
    let mut buf: Vec<u8> = vec![0u8; n];
    let mut sum = sum;
    for i in 0..n {
      let d = sum.min(9);
      buf[i] = b'0' + d as u8;
      sum -= d;
    }
    if sum > 0 {
      return "".to_string();
    }
    String::from_utf8(buf).unwrap()
  }
}
