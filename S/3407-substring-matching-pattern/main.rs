impl Solution {
  pub fn has_match(s: String, p: String) -> bool {
    let sb = s.as_bytes().to_vec();
    let pb = p.as_bytes().to_vec();

    let mut sb_idx: usize = 0;
    let mut pb_idx: usize = 0;
    while sb_idx < sb.len() && pb_idx < pb.len() {
      if pb[pb_idx] == b'*' {
        pb_idx += 1;
        continue;
      }
      let mut nsb = sb_idx;
      let mut npb = pb_idx;
      while nsb < sb.len() && npb < pb.len() && sb[nsb] == pb[npb] {
        nsb += 1;
        npb += 1;
      }
      if npb == pb.len() || pb[npb] == b'*' {
        sb_idx = nsb;
        pb_idx = npb;
      } else {
        sb_idx += 1;
      }
    }
    pb_idx == pb.len() || pb_idx == pb.len() - 1 && pb[pb_idx] == b'*'
  }
}
