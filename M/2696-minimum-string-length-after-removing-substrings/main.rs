struct Solution {}

impl Solution {
  pub fn min_length(s: String) -> i32 {
    let mut stack: Vec<u8> = Vec::new();
    s.as_bytes().iter().for_each(|&v| {
      stack.push(v);
      while stack.len() > 1
        && (stack[stack.len() - 2] == b'A' && stack[stack.len() - 1] == b'B'
          || stack[stack.len() - 2] == b'C' && stack[stack.len() - 1] == b'D')
      {
        stack.pop();
        stack.pop();
      }
    });
    stack.len() as i32
  }
}
