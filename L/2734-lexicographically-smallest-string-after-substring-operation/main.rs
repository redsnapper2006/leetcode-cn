struct Solution {}

impl Solution {
  pub fn smallest_string(s: String) -> String {
    let mut buf = s.as_bytes().to_vec();
    let mut idx: usize = 0;
    while idx < buf.len() && buf[idx] == b'a' {
      idx += 1;
    }
    if idx == buf.len() {
      buf[idx - 1] = b'z';
    } else {
      while idx < buf.len() && buf[idx] != b'a' {
        buf[idx] -= 1;
        idx += 1;
      }
    }

    String::from_utf8(buf).unwrap()
  }
}
