impl Solution {
  pub fn minimize_string_value(s: String) -> String {
    let mut bb: Vec<u8> = s.as_bytes().to_vec();
    let mut buf: Vec<i32> = vec![0; 26];
    for i in 0..bb.len() {
      if bb[i] != b'?' {
        buf[(bb[i] - b'a') as usize] += 1;
      }
    }

    let mut bk: Vec<u8> = vec![];
    for i in 0..bb.len() {
      if bb[i] == b'?' {
        let mut max: i32 = i32::MAX;
        let mut idx: usize = 0;
        for ii in 0..26 {
          if buf[ii] < max {
            idx = ii;
            max = buf[ii];
          }
        }
        bk.push(b'a' + idx as u8);
        buf[idx] += 1;
      }
    }
    bk.sort_unstable();
    let mut idx: usize = 0;
    for i in 0..bb.len() {
      if bb[i] == b'?' {
        bb[i] = bk[idx];
        idx += 1;
      }
    }
    String::from_utf8(bb).unwrap()
  }
}
