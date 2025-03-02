impl Solution {
  pub fn is_balanced(num: String) -> bool {
    num.as_bytes().iter().enumerate().fold(0, |sum, (idx, v)| {
      sum + (if idx % 2 == 0 { 1 } else { -1 }) * (v - b'0') as i32
    }) == 0
  }
}
