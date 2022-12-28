struct Solution {}

impl Solution {
  pub fn minimum_length(s: String) -> i32 {
    let bb: Vec<u8> = s.as_bytes().to_vec();
    let mut start: usize = 0;
    let mut end: usize = bb.len() - 1;
    while start < end {
      let s: u8 = bb[start];
      let e: u8 = bb[end];
      if s != e {
        break;
      }
      while start <= end && bb[start] == s {
        start += 1;
      }
      while end >= start && bb[end] == e {
        end -= 1;
      }
    }
    let mut ret: i32 = (end - start) as i32;
    if ret < 0 {
      return 0;
    }
    ret + 1
  }
}
