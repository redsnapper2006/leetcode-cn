struct Solution {}

impl Solution {
  pub fn remove_stars(s: String) -> String {
    let mut stack: Vec<u8> = Vec::new();

    for b in s.as_bytes() {
      if *b != '*' as u8 {
        stack.push(*b);
      } else {
        stack.pop();
      }
    }
    String::from_utf8(stack).unwrap()
  }
}
