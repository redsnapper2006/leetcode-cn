impl Solution {
  pub fn rearrange_string(s: String, x: char, y: char) -> String {
    let bb = s.as_bytes().to_vec();
    let mut buf: Vec<u8> = vec![y as u8 - 'a' as u8 + b'a'; bb.len()];
    let mut start: usize = 0;
    let mut end: usize = bb.len() - 1;
    bb.iter().for_each(|&b| {
      let idx = if b == x as u8 - 'a' as u8 + b'a' {
        end -= 1;
        end + 1
      } else {
        start += 1;
        start - 1
      };
      buf[idx] = b;
    });

    String::from_utf8(buf).unwrap()
  }
}
