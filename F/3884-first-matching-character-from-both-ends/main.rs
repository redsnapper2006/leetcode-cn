impl Solution {
  pub fn first_matching_index(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    for i in 0..(bb.len() + 1) / 2 {
      if bb[i] == bb[bb.len() - 1 - i] {
        return i as _;
      }
    }
    -1
  }
}
