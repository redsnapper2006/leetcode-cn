struct Solution {}

impl Solution {
  pub fn minimum_moves(s: String) -> i32 {
    let mut idx: usize = 0;
    let mut ret: i32 = 0;
    while idx < s.len() {
      if s.as_bytes()[idx] == 'X' as u8 {
        idx += 3;
        ret += 1;
      } else {
        idx += 1;
      }
    }
    ret
  }
}
