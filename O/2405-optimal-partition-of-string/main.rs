// struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn partition_string(s: String) -> i32 {
    let mut cnt: i32 = 0;
    let mut cached: HashSet<u8> = HashSet::new();

    for b in s.as_bytes() {
      if !cached.contains(&b) {
        cached.insert(b);
        continue;
      }
      cnt += 1;
      cached.clear();
      cached.insert(b);
    }
    cnt + 1
  }
}
