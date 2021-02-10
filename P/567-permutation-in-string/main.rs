struct Solution {}

impl Solution {
  pub fn is_match(b1: &[i32; 26], b2: &[i32; 26]) -> bool {
    for i in 0..26 {
      if b1[i] != b2[i] {
        return false;
      }
    }
    true
  }

  pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
      return false;
    }
    let bb1 = s1.as_bytes();
    let bb2 = s2.as_bytes();

    let mut b1: [i32; 26] = [0; 26];
    let mut b2: [i32; 26] = [0; 26];
    for i in 0..bb1.len() {
      b1[(bb1[i] - 'a' as u8) as usize] += 1;
      b2[(bb2[i] - 'a' as u8) as usize] += 1;
    }
    if Solution::is_match(&b1, &b2) {
      return true;
    }
    for i in 1..bb2.len() - bb1.len() + 1 {
      b2[(bb2[i - 1] - 'a' as u8) as usize] -= 1;
      b2[(bb2[i + bb1.len() - 1] - 'a' as u8) as usize] += 1;
      if Solution::is_match(&b1, &b2) {
        return true;
      }
    }
    false
  }
}

fn main() {
  println!("{}", Solution::check_inclusion("ab", "eidbaooo"))
}
