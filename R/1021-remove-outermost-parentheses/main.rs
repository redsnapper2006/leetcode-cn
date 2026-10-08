impl Solution {
  pub fn remove_outer_parentheses(s: String) -> String {
    let mut ans: Vec<u8> = Vec::with_capacity(s.len());
    let mut depth = 0i32;
    for b in s.bytes() {
      depth += if b == b')' { -1 } else { 0 };
      if depth > 0 {
        ans.push(b);
      }
      depth += if b == b'(' { 1 } else { 0 };
    }
    String::from_utf8(ans).unwrap()
  }
}
