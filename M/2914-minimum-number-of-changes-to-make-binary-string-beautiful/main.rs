impl Solution {
  pub fn min_changes(s: String) -> i32 {
    s.as_bytes().chunks_exact(2).filter(|pair| pair[0] != pair[1]).count() as i32
  }
}
