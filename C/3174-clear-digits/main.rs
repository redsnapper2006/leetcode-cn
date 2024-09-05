struct Solution {}

impl Solution {
  pub fn clear_digits(s: String) -> String {
    let bb = s.as_bytes().to_vec();
    let mut ans: Vec<u8> = Vec::new();

    bb.iter().enumerate().for_each(|(idx, &b)| {
      if b >= b'0' && b <= b'9' && ans.len() > 0 {
        ans.pop();
      } else {
        ans.push(b);
      }
    });

    String::from_utf8(ans).unwrap()
  }
}
