struct Solution {}

impl Solution {
  pub fn get_smallest_string(s: String) -> String {
    let mut bb = s.as_bytes().to_vec();

    let mut idx: usize = 0;
    while idx < bb.len() - 1 {
      let b1 = (bb[idx] - b'0') as i32;
      let b2 = (bb[idx + 1] - b'0') as i32;
      if b1 % 2 == b2 % 2 && b2 < b1 {
        let t = bb[idx + 1];
        bb[idx + 1] = bb[idx];
        bb[idx] = t;
        break;
      }
      idx += 1;
    }
    String::from_utf8(bb).unwrap()
  }
}
