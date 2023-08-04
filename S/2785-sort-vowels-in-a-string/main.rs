impl Solution {
  pub fn sort_vowels(s: String) -> String {
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
