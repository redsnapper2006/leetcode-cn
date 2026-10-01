impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<u8> = Vec::with_capacity(s.len() / 2);

    for b in s.bytes() {
      match b {
        b')' | b']' | b'}' => {
          let Some(p) = stack.pop() else { return false };
          if !matches!((p, b), (b'(', b')') | (b'[', b']') | (b'{', b'}')) {
            return false;
          }
        }
        _ => stack.push(b),
      }
    }
    stack.is_empty()
  }
}
