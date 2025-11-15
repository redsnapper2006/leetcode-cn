use std::collections::HashMap;

impl Solution {
  pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let bb = s.as_bytes().to_vec();
    let mut m: HashMap<i32, (usize, usize)> = HashMap::new();
    (1..32.min(bb.len() + 1)).for_each(|offset| {
      let mut t: HashMap<i32, (usize, usize)> = HashMap::new();
      let mut base: i32 = 0;
      for i in 0..offset {
        base = base * 2 + (bb[i] - b'0') as i32;
      }
      t.insert(base, (0, offset - 1));
      for i in offset..bb.len() {
        base = ((1 << offset) - 1) & (base * 2 + (bb[i] - b'0') as i32);
        if t.contains_key(&base) || m.contains_key(&base) {
          continue;
        }
        t.insert(base, (i - offset + 1, i));
        if t.len() == (1 << offset) {
          break;
        }
      }
      for (&k, &v) in t.iter() {
        if m.contains_key(&k) {
          continue;
        }
        m.insert(k, v);
      }
    });

    let mut ret: Vec<Vec<i32>> = vec![];
    queries.iter().for_each(|q| {
      let v = q[0] ^ q[1];
      if !m.contains_key(&v) {
        ret.push(vec![-1, -1]);
      } else {
        let off = m.get(&v).unwrap();
        ret.push(vec![off.0 as i32, off.1 as i32]);
      }
    });
    ret
  }
}
