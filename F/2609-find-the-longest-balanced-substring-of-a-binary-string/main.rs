impl Solution {
  pub fn find_the_longest_balanced_substring(s: String) -> i32 {
    let buf: Vec<u8> = s.bytes().collect();

    let mut start: usize = 0;
    let mut zero: i32 = 0;
    let mut one: i32 = 0;
    let mut ret: i32 = 0;
    while start < buf.len() {
      if buf[start] == b'1' {
        start += 1;
      } else {
        zero = 0;
        while start < buf.len() && buf[start] == b'0' {
          start += 1;
          zero += 1;
        }
        one = 0;
        while start < buf.len() && buf[start] == b'1' {
          start += 1;
          one += 1;
        }
        let m = one.min(zero);
        if ret < m {
          ret = m;
        }
      }
    }
    ret * 2
  }
}
