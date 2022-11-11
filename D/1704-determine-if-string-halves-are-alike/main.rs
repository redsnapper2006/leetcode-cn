struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn halves_are_alike(s: String) -> bool {
    let bb = s.as_bytes().to_vec();
    let keys: Vec<u8> = vec![
      'a' as u8, 'e' as u8, 'i' as u8, 'o' as u8, 'u' as u8, 'A' as u8, 'E' as u8, 'I' as u8,
      'O' as u8, 'U' as u8,
    ];
    let mut idx_m: HashMap<u8, usize> = HashMap::new();
    for i in 0..keys.len() {
      idx_m.insert(keys[i], i);
    }

    let size = bb.len() / 2;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    for i in 0..size {
      let offset = idx_m.get(&bb[i]);
      if offset.is_some() {
        left += 1;
      }

      let offset2 = idx_m.get(&bb[i + size]);
      if offset2.is_some() {
        right += 1;
      }
    }

    // println!("{:?} {:?}", left, right);
    let ret = left == right;
    ret
  }
}

fn main() {
  println!("{}", Solution::halves_are_alike("abcd".to_string()))
}
