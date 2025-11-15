impl Solution {
  pub fn score_balance(s: String) -> bool {
    let bb = s.as_bytes().to_vec();
    let mut sum: i32 = 0;
    let buf = bb
      .iter()
      .map(|b| {
        sum += (b - b'a') as i32 + 1;
        sum
      })
      .collect::<Vec<i32>>();

    buf[buf.len() - 1] % 2 == 0
      && match buf.binary_search(&(buf[buf.len() - 1] / 2)) {
        Ok(_) => true,
        Err(_) => false,
      }
  }
}
