struct Solution {}

impl Solution {
  pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let mut idx: usize = 0;

    let b1: Vec<u8> = s1.into_bytes();
    let b2: Vec<u8> = s2.into_bytes();
    let b3: Vec<u8> = s3.into_bytes();

    while idx < b1.len() && idx < b2.len() && idx < b3.len() {
      if b1[idx] != b2[idx] || b2[idx] != b3[idx] {
        break;
      }
      idx += 1;
    }

    match idx {
      0 => -1,
      _ => (b1.len() - idx + b2.len() - idx + b3.len() - idx) as i32,
    }
  }
}
