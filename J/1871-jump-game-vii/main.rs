impl Solution {
  pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
    let bb = s.as_bytes().to_vec();
    let mut buf: Vec<i32> = vec![0; bb.len()];
    buf[0] = 1;
    buf[1] = -1;
    for i in 0..bb.len() {
      buf[i] += if i > 0 { buf[i - 1] } else { 0 };
      if bb[i] == b'1' || buf[i] == 0 {
        continue;
      }
      let n = i + min_jump as usize;
      if n < bb.len() {
        buf[n] += 1;
      }
      let nn = i + max_jump as usize + 1;
      if nn < bb.len() {
        buf[nn] -= 1;
      }
    }
    bb[bb.len() - 1] == b'0' && buf[buf.len() - 1] > 0
  }
}
