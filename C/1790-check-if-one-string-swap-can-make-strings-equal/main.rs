struct Solution {}

impl Solution {
  pub fn are_almost_equal(s1: String, s2: String) -> bool {
    let b1: Vec<u8> = s1.as_bytes().to_vec();
    let b2: Vec<u8> = s2.as_bytes().to_vec();
    let mut idx: Vec<usize> = Vec::new();
    for i in 0..b1.len() {
      if b1[i] != b2[i] {
        idx.push(i);
      }
    }
    if idx.len() == 1 || idx.len() > 2 {
      return false;
    }
    if idx.len() == 0 {
      return true;
    }
    b1[idx[0]] == b2[idx[1]] && b1[idx[1]] == b2[idx[0]]
  }
}

fn main() {
  println!(
    "{}",
    Solution::are_almost_equal(String::from("bank"), String::from("kanb"))
  );
}
