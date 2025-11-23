impl Solution {
  pub fn min_length_after_removals(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut stack: Vec<u8> = vec![];
    bb.iter().for_each(|&b| {
      if stack.len() == 0 && b == stack[stack.len() - 1] {
        stack.push(b);
      } else {
        stack.pop();
      }
    });
    stack.len() as _
  }
}
