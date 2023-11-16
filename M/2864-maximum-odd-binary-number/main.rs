struct Solution {}

impl Solution {
  pub fn maximum_odd_binary_number(s: String) -> String {
    let mut one: i32 = 0;
    let mut zero: i32 = 0;
    s.as_bytes().iter().for_each(|&c| match c {
      b'1' => one += 1,
      _ => zero += 1,
    });
    let mut buf: Vec<u8> = Vec::new();
    (0..one - 1).for_each(|_| buf.push(b'1'));
    (0..zero).for_each(|_| buf.push(b'0'));
    buf.push(b'1');
    String::from_utf8(buf).unwrap()
  }
}
