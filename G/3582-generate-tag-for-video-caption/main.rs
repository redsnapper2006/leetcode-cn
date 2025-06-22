impl Solution {
  pub fn generate_tag(caption: String) -> String {
    let bb: Vec<u8> = caption.trim().as_bytes().to_vec();

    let mut ans: Vec<u8> = vec![b'#'];
    let mut is_space: bool = false;
    (0..bb.len()).for_each(|idx| {
      if idx == 0 {
        if bb[idx] >= b'A' && bb[idx] <= b'Z' {
          ans.push((bb[idx] - b'A' + b'a') as u8);
        } else {
          ans.push(bb[idx]);
        }
      } else {
        if bb[idx] == b' ' {
          is_space = true;
        } else {
          let mut b = bb[idx];
          if b >= b'A' && b <= b'Z' {
            b = b - b'A' + b'a';
          }
          if is_space {
            ans.push((b - b'a' + b'A') as u8);
            is_space = false;
          } else {
            ans.push(b);
          }
        }
      }
    });

    String::from_utf8(ans[0..100.min(ans.len())].to_vec()).unwrap()
  }
}
