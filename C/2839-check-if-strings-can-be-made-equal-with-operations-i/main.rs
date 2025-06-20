impl Solution {
  pub fn can_be_equal(s1: String, s2: String) -> bool {
    let b1 = s1.as_bytes().to_vec();
    let b2 = s2.as_bytes().to_vec();
    (b1[0] == b2[0] && b1[2] == b2[2] || b1[0] == b2[2] && b1[2] == b2[0])
      && (b1[1] == b2[1] && b1[3] == b2[3] || b1[1] == b2[3] && b1[3] == b2[1])
  }
}
