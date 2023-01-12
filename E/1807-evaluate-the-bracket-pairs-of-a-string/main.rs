struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let mut is_in: bool = false;
    let mut buf: Vec<u8> = Vec::new();
    let mut ret: Vec<u8> = Vec::new();

    let mut m: HashMap<String, String> = knowledge
      .into_iter()
      .map(|x| (x[0].clone(), x[1].clone()))
      .collect();
    for b in s.as_bytes() {
      if *b == '(' as u8 {
        is_in = true;
        buf.clear();
      } else if *b == ')' as u8 {
        is_in = false;
        let k = String::from_utf8(buf.clone()).unwrap();
        if m.contains_key(&k) {
          ret.append(&mut m.get(&k).unwrap().as_bytes().to_vec());
        } else {
          ret.push('?' as u8);
        }
      } else {
        if is_in {
          buf.push(*b);
        } else {
          ret.push(*b);
        }
      }
    }
    String::from_utf8(ret).unwrap()
  }
}
