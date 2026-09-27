impl Solution {
  pub fn reverse_parentheses(s: String) -> String {
    let mut stack: Vec<usize> = vec![];
    let mut buf: Vec<u8> = Vec::with_capacity(s.len());
    for b in s.bytes() {
      match b {
        b'(' => stack.push(buf.len()),
        b')' => {
          let start = stack.pop().unwrap();
          buf[start..].reverse();
        }
        _ => buf.push(b),
      }
    }
    String::from_utf8(buf).unwrap()
  }
}
