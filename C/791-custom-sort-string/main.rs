struct Solution {}

use std::cmp::Ordering;
use std::collections::HashMap;
impl Solution {
  pub fn custom_sort_string(order: String, s: String) -> String {
    let mut m: HashMap<u8, usize> = HashMap::new();
    for (i, b) in order.as_bytes().iter().enumerate() {
      m.insert(*b, i + 1);
    }

    let mut bb = s.as_bytes().to_vec();
    bb.sort_by(|a, b| {
      if !m.contains_key(&a) {
        Ordering::Greater
      } else if !m.contains_key(&b) {
        Ordering::Less
      } else {
        let idxA = m.get(&a).unwrap();
        let idxB = m.get(&b).unwrap();
        if idxA < idxB {
          Ordering::Less
        } else if idxA == idxB {
          Ordering::Equal
        } else {
          Ordering::Greater
        }
      }
    });

    String::from_utf8(bb).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::custom_sort_string("abc".to_string(), "b".to_string())
  );
}
