
use std::collections::HashSet;
impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
      let bb = s.as_bytes().to_vec();

      let mut right : usize = 0;
      let mut left :usize = 0;
      let mut base: Vec<u8> = vec![];
      let mut m : HashSet<String> =HashSet::new();
      let mut ans : Vec<String> = vec![];
      while right < bb.len() {
        base.push(bb[right]);
        let ts = String::from_utf8(base.clone()).unwrap();
        if !m.contains(&ts) {
          m.insert(ts.clone());
          ans.push(ts);
          left = right + 1;
          base = vec![];
        }
        right += 1;
      }
      ans
    }
}
