struct Solution {}

impl Solution {
  pub fn digit_count(num: String) -> bool {
    let mut digits: [i32; 10] = [0; 10];
    num
      .as_bytes()
      .iter()
      .for_each(|b| digits[(b - '0' as u8) as usize] += 1);

    num
      .as_bytes()
      .iter()
      .enumerate()
      .map(|(i, d)| digits[i] == (*d - '0' as u8) as i32)
      .all(|x| x)
  }
}
