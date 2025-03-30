impl Solution {
  pub fn find_valid_pair(s: String) -> String {
    let mut buf: Vec<i32> = vec![0; 10];

    let bb = s.as_bytes().to_vec();
    bb.iter().for_each(|b| {
      buf[(b - b'0') as usize] += 1;
    });

    let mut idx: usize = 1;
    while idx < bb.len() {
      if bb[idx - 1] != bb[idx]
        && (bb[idx - 1] - b'0') as i32 == buf[(bb[idx - 1] - b'0') as usize]
        && (bb[idx] - b'0') as i32 == buf[(bb[idx] - b'0') as usize]
      {
        return String::from_utf8(bb[idx - 1..=idx].to_vec()).unwrap();
      }

      idx += 1;
    }

    "".to_string()
  }
}
