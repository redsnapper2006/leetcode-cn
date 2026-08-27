impl Solution {
  pub fn lex_greater_permutation(s: String, target: String) -> String {
    let mut cnt: [u16; 26] = [0; 26];

    let bs = s.as_bytes();
    for &b in bs {
      cnt[(b - b'a') as usize] += 1;
    }

    let bt = target.as_bytes();
    let mut buf: Vec<u8> = Vec::with_capacity(bs.len());
    fn recur(bt: &[u8], idx: usize, cnt: &mut [u16], buf: &mut Vec<u8>) -> bool {
      if idx == bt.len() {
        return false;
      }

      let offset = (bt[idx] - b'a') as usize;
      if cnt[offset] > 0 {
        cnt[offset] -= 1;
        buf.push(bt[idx]);
        if recur(bt, idx + 1, cnt, buf) {
          return true;
        }
        cnt[offset] += 1;
        buf.pop();
      }
      for i in offset + 1..26 {
        if cnt[i] > 0 {
          cnt[i] -= 1;
          buf.push(b'a' + i as u8);
          for j in 0..26 {
            buf.extend(std::iter::repeat(b'a' + j as u8).take(cnt[j] as usize));
          }
          return true;
        }
      }
      false
    }

    if recur(&bt, 0, &mut cnt, &mut buf) {
      String::from_utf8(buf).unwrap()
    } else {
      "".to_string()
    }
  }
}
