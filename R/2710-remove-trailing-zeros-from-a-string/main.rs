struct Solution {}

impl Solution {
  pub fn remove_trailing_zeros(num: String) -> String {
    let mut buf: Vec<u8> = num.as_bytes().to_vec();

    let mut idx: usize = buf.len() - 1;
    while idx >= 0 && buf[idx] == '0' as u8 {
      idx -= 1;
    }

    buf.resize(idx + 1, ' ' as u8);
    String::from_utf8(buf).unwrap()
  }
}
