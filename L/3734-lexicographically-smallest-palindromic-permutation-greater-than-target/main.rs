impl Solution {
  pub fn lex_palindromic_permutation(s: String, target: String) -> String {
    let mut cnt: [u16; 26] = [0; 26];

    let bs = s.as_bytes();
    for &b in bs {
      cnt[(b - b'a') as usize] += 1;
    }
    let mut odd: u16 = 0;
    let mut ob: u8 = b' ';
    for i in 0..26 {
      odd += cnt[i] % 2;
      if cnt[i] % 2 == 1 {
        ob = b'a' + i as u8;
      }
    }
    if odd > 1 {
      return "".to_string();
    }

    let bt = target.as_bytes();
    let mut buf: Vec<u8> = vec![b'0'; bt.len()];
    if odd == 1 {
      cnt[(ob - b'a') as usize] -= 1;
      buf[bt.len() / 2] = ob;
    }
    fn recur(bt: &[u8], idx: usize, cnt: &mut [u16], buf: &mut Vec<u8>) -> bool {
      if idx == bt.len() / 2 {
        for i in 0..bt.len() {
          if buf[i] != bt[i] {
            return buf[i] > bt[i];
          }
        }
        return false;
      }

      let offset = (bt[idx] - b'a') as usize;
      if cnt[offset] > 1 {
        cnt[offset] -= 2;
        let candi = b'a' + offset as u8;
        buf[idx] = candi;
        buf[bt.len() - 1 - idx] = candi;
        if recur(bt, idx + 1, cnt, buf) {
          return true;
        }
        cnt[offset] += 2;
        buf[idx] = b'0';
        buf[bt.len() - 1 - idx] = b'0';
      }

      for i in offset + 1..26 {
        if cnt[i] > 1 {
          cnt[i] -= 2;
          let candi = b'a' + i as u8;
          buf[idx] = candi;
          buf[bt.len() - 1 - idx] = candi;
          let mut nxt: usize = idx + 1;
          for j in 0..26 {
            let candi = b'a' + j as u8;
            for _ in 0..cnt[j] / 2 {
              buf[nxt] = candi;
              buf[bt.len() - 1 - nxt] = candi;
              nxt += 1;
            }
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

struct Solution {}

fn main() {
  // println!(
  //   "{}",
  //   Solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string())
  // );

  // println!(
  //   "{}",
  //   Solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string())
  // );
  // println!(
  //   "{}",
  //   Solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string())
  // );
  println!(
    "{}",
    Solution::lex_palindromic_permutation("abb".to_string(), "bba".to_string())
  );
}
