use std::collections::HashSet;
impl Solution {
  pub fn clear_stars(s: String) -> String {
    let bb = s.as_bytes().to_vec();
    let mut buf: Vec<Vec<usize>> = vec![vec![]; 26];
    let mut removed: HashSet<usize> = HashSet::new();

    bb.iter().enumerate().for_each(|(idx, b)| {
      if *b == b'*' {
        for ii in 0..26 {
          if buf[ii].len() == 0 {
            continue;
          }
          removed.insert(buf[ii].pop().unwrap());
          break;
        }
      } else {
        buf[(b - b'a') as usize].push(idx);
      }
    });

    let mut ans: Vec<u8> = vec![];
    bb.iter().enumerate().for_each(|(idx, b)| {
      if *b == b'*' || removed.contains(&idx) {
        return;
      }
      ans.push(*b);
    });
    String::from_utf8(ans).unwrap()
  }
}
