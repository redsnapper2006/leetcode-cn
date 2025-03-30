impl Solution {
  pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut buf: Vec<u8> = vec![];
    let bb = s.as_bytes().to_vec();
    let mut space_idx: i32 = spaces.len() as i32 - 1;
    (0..bb.len()).rev().for_each(|idx| {
      buf.push(bb[idx]);
      if space_idx >= 0 && idx == spaces[space_idx as usize] as usize {
        buf.push(b' ');
        space_idx -= 1;
      }
    });
    buf.reverse();
    String::from_utf8(buf).unwrap()
  }
}
