struct Solution {}

impl Solution {
  pub fn find_replace_string(
    s: String,
    indices: Vec<i32>,
    sources: Vec<String>,
    targets: Vec<String>,
  ) -> String {
    let mut bb: Vec<u8> = s.as_bytes().to_vec();
    let mut replace: Vec<i32> = vec![0; indices.len()];
    (0..indices.len()).for_each(|i| {
      let mut offset = indices[i] as usize;
      let mut end = offset + sources[i].len();
      while offset < bb.len()
        && offset < end
        && bb[offset] == sources[i].as_bytes()[offset - indices[i] as usize]
      {
        offset += 1;
      }
      if offset == end {
        replace[i] = 1;
        bb[indices[i] as usize] = 0;
      }
    });
    let mut res: Vec<u8> = vec![];
    let mut idx: usize = 0;
    while idx < bb.len() {
      if bb[idx] == 0 {
        let i = indices.iter().position(|&x| x as usize == idx).unwrap();
        res.extend(targets[i].as_bytes());
        idx += sources[i].len();
      } else {
        res.push(bb[idx]);
        idx += 1;
      }
    }
    String::from_utf8(res).unwrap()
  }
}
