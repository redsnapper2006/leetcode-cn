impl Solution {
  pub fn count_valid_substrings(s: String) -> i64 {
    let bb = s.as_bytes();
    let mut b_pos = 0usize;
    let mut a_pos: Vec<usize> = vec![];
    let mut prev = 0usize;

    let mut ans = 0i64;
    for i in 0..bb.len() {
      if bb[i] == b'b' {
        b_pos = i;
      }
      if bb[i] == b'a' {
        a_pos.push(i);
      }
      if bb[i] == b'c' {
        while a_pos.len() > 0 && a_pos[0] < b_pos {
          prev = a_pos[0] + 1;
          a_pos.remove(0);
        }
      }
      ans += (i - prev + 1) as i64;
    }

    ans
  }
}
