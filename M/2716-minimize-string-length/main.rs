use std::collections::HashSet;
impl Solution {
  pub fn minimized_string_length(s: String) -> i32 {
    HashSet::<u8>::from_iter(s.bytes()).len() as i32
  }
}
