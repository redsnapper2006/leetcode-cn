impl Solution {
  pub fn count_seniors(details: Vec<String>) -> i32 {
    details
      .iter()
      .filter(|&x| x.as_bytes()[11] > b'6' || x.as_bytes()[11] == b'6' && x.as_bytes()[12] > b'0')
      .count() as i32
  }
}
