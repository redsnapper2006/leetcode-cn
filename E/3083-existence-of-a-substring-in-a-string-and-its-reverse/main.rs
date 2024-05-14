struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn is_substring_present(s: String) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    let bb = s.as_bytes().to_vec();
    (1..bb.len()).for_each(|idx| {
      set.insert((bb[idx - 1] - b'a') as i32 * 26 + (bb[idx] - b'a') as i32);
    });

    let mut ans: bool = false;
    (0..bb.len() - 1).rev().for_each(|idx| {
      ans |= set.contains(&((bb[idx + 1] - b'a') as i32 * 26 + (bb[idx] - b'a') as i32));
    });

    ans
  }
}
