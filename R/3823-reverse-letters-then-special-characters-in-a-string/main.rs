impl Solution {
  pub fn reverse_by_type(s: String) -> String {
    let mut bb: Vec<u8> = s.as_bytes().to_vec();

    fn change(bb: &mut Vec<u8>, valid: fn(u8) -> bool) {
      let mut start: usize = 0;
      let mut end: usize = bb.len() - 1;
      while start < end {
        while start < end && valid(bb[start]) {
          start += 1;
        }
        while start < end && valid(bb[end]) {
          if end == 0 {
            break;
          }
          end -= 1;
        }
        if start < end {
          let t = bb[start];
          bb[start] = bb[end];
          bb[end] = t;
          start += 1;
          end -= 1;
        }
      }
    }

    fn letter(a: u8) -> bool {
      a >= b'a' && a <= b'z'
    };

    fn special(a: u8) -> bool {
      !(a >= b'a' && a <= b'z')
    };

    change(&mut bb, letter);
    change(&mut bb, special);
    String::from_utf8(bb).unwrap()
  }
}
