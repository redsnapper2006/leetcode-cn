impl Solution {
  pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
    let b1 = coordinate1.as_bytes().to_vec();
    let b2 = coordinate2.as_bytes().to_vec();
    (b1[0] - b'a' + b1[1] - b'1') % 2 == (b2[0] - b'a' + b2[1] - b'1') % 2
  }
}
