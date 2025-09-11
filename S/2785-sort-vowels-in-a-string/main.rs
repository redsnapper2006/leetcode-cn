impl Solution {
  pub fn sort_vowels(s: String) -> String {
    let mut res: Vec<u8> = Vec::new();
    let mut cnt: Vec<i32> = vec![0; (b'z' - b'A') as usize + 1];
    s.as_bytes().iter().for_each(|&v| {
      if "AEIOUaeiou".contains(v as char) {
        cnt[(v - b'A') as usize] += 1;
      }
    });

    let mut off: usize = 0;
    s.as_bytes().iter().for_each(|&v| {
      if "AEIOUaeiou".contains(v as char) {
        while cnt[off] == 0 {
          off += 1;
        }
        cnt[off] -= 1;
        res.push((b'A' + off as u8));
      } else {
        res.push(v);
      }
    });

    String::from_utf8(res).unwrap()
  }

  pub fn sort_vowels2(s: String) -> String {
    let mut res: Vec<u8> = Vec::new();
    let mut idxs: Vec<usize> = Vec::new();
    let mut vowels: Vec<u8> = Vec::new();
    s.as_bytes().iter().enumerate().for_each(|(idx, &v)| {
      if v == b'a'
        || v == b'e'
        || v == b'i'
        || v == b'o'
        || v == b'u'
        || v == b'A'
        || v == b'E'
        || v == b'I'
        || v == b'O'
        || v == b'U'
      {
        vowels.push(v);
        idxs.push(idx);
      }

      res.push(v);
    });
    vowels.sort();
    idxs.iter().enumerate().for_each(|(i, v)| {
      res[*v] = vowels[i];
    });
    String::from_utf8(res).unwrap()
  }
}
