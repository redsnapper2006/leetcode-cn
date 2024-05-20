
use std::collections::HashMap;
impl Solution {
  pub fn longest_awesome(s: String) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(0, -1);
    let mut prev: i32 = 0;
    let mut ans: i32 = 0;
    s.as_bytes().iter().enumerate().for_each(|(idx, &c)| {
      prev ^= 1 << (c - b'0') as i32;
      if !m.contains_key(&prev) {
        m.insert(prev, idx as i32);
      } else {
        ans = ans.max(idx as i32 - m.get(&prev).unwrap());
      }
      (0..10).for_each(|i| {
        let mask = 1 << i;
        if m.contains_key(&(prev ^ mask)) {
          ans = ans.max(idx as i32 - m.get(&(prev ^ mask)).unwrap());
        }
      });
    });
    ans
  }
}
