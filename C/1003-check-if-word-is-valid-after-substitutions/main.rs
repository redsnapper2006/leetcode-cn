struct Solution {}

impl Solution {
  pub fn is_valid(s: String) -> bool {
    let bb: Vec<u8> = s.as_bytes().to_vec();

    let mut stack: Vec<u8> = Vec::new();
    bb.iter().for_each(|&v| {
      stack.push(v);

      let size = stack.len();
      if size >= 3
        && stack[size - 1] == 'c' as u8
        && stack[size - 2] == 'b' as u8
        && stack[size - 3] == 'a' as u8
      {
        stack.pop();
        stack.pop();
        stack.pop();
      }
    });

    stack.len() == 0
  }
}
