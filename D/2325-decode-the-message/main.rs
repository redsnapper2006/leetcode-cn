struct Solution {}

impl Solution {
  pub fn decode_message(key: String, message: String) -> String {
    let mut m: [u8; 26] = [0; 26];

    let mut idx: usize = 0;
    for b in key.as_bytes() {
      if *b == ' ' as u8 {
        continue;
      }
      let diff: usize = (*b - 'a' as u8) as usize;
      if m[diff] == 0 {
        m[diff] = ('a' as u8 + idx as u8) as u8;
        idx += 1;
      }
    }

    let mut buf: Vec<u8> = Vec::new();
    for mb in message.as_bytes() {
      if *mb == ' ' as u8 {
        buf.push(*mb);
        continue;
      }
      let diff = (mb - 'a' as u8) as usize;
      buf.push(m[diff]);
    }
    String::from_utf8(buf).unwrap()
  }
}
