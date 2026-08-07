use std::collections::HashMap;

impl Solution {
  pub fn majority_frequency_group(s: String) -> String {
    let mut buf: [i32; 26] = [0; 26];
    s.as_bytes().iter().for_each(|b| {
      buf[(b - b'a') as usize] += 1;
    });

    let mut m: HashMap<i32, Vec<u8>> = HashMap::new();
    for i in 0..26 {
      if buf[i] == 0 {
        continue;
      }
      m.entry(buf[i]).or_insert(vec![]).push(b'a' + i as u8);
    }

    let mut ans: Vec<u8> = vec![];
    let mut feq: i32 = 0;
    for (k, v) in m {
      if v.len() > ans.len() || v.len() == ans.len() && k > feq {
        feq = k;
        ans = v.clone();
      }
    }
    String::from_utf8(ans).unwrap()
  }
}
