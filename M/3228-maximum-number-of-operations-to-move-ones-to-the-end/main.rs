impl Solution {
  pub fn max_operations(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut idx: usize = 0;
    let mut cnt: i32 = 0;
    let mut ans: i32 = 0;
    while idx < bb.len() {
      if bb[idx] == b'1' {
        cnt += 1;
        idx += 1;
      } else {
        while idx < bb.len() && bb[idx] == b'0' {
          idx += 1;
        }
        ans += cnt;
      }
    }
    ans
  }
}
