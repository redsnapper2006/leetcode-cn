impl Solution {
  pub fn is_adjacent_diff_at_most_two(s: String) -> bool {
    let bb = s.as_bytes().to_vec();
    for idx in 1..bb.len() {
      if (bb[idx] as i32 - bb[idx - 1] as i32).abs() > 2 {
        return false;
      }
    }
    true
  }
}
