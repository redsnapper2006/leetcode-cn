impl Solution {
  pub fn largest_even(s: String) -> String {
    let buf: Vec<u8> = s.as_bytes().to_vec();
    let mut s: usize = buf.len();
    while s > 0 && buf[s - 1] == b'1' {
      s -= 1;
    }
    String::from_utf8(buf[0..s].to_vec()).unwrap()
  }
}
