impl Solution {
  pub fn sort_vowels(s: String) -> String {
    let mut bb = s.as_bytes().to_vec();
    let mut buf: Vec<(u8, i32, usize)> = vec![
      (b'a', 0, bb.len()),
      (b'e', 0, bb.len()),
      (b'i', 0, bb.len()),
      (b'o', 0, bb.len()),
      (b'u', 0, bb.len()),
    ];

    bb.iter().enumerate().for_each(|(idx, &b)| {
      let ii: usize = match b {
        b'a' => 0,
        b'e' => 1,
        b'i' => 2,
        b'o' => 3,
        b'u' => 4,
        _ => 5,
      };

      if ii < 5 {
        buf[ii].1 += 1;
        if buf[ii].2 == bb.len() {
          buf[ii].2 = idx;
        }
      }
    });
    buf.sort_by(|x, y| {
      if x.1 != y.1 {
        return y.1.cmp(&x.1);
      }
      x.2.cmp(&y.2)
    });
    let mut idx: usize = 0;
    for i in 0..bb.len() {
      match bb[i] {
        b'a' | b'e' | b'i' | b'o' | b'u' => {
          bb[i] = buf[idx].0;
          buf[idx].1 -= 1;
          if buf[idx].1 == 0 {
            idx += 1;
          }
        }
        _ => {}
      };
    }
    String::from_utf8(bb).unwrap()
  }
}
