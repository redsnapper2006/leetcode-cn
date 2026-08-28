impl Solution {
  pub fn lex_palindromic_permutation(s: String, target: String) -> String {
    let mut cnt: [u16; 26] = [0; 26];
    for &b in s.as_bytes() {
      cnt[(b - b'a') as usize] += 1;
    }

    let (mut odd, mut ob) = (0u16, 0u8);
    for i in 0..26 {
      if cnt[i] & 1 == 1 {
        odd += 1;
        ob = b'a' + i as u8;
      }
    }
    if odd > 1 {
      return "".to_string();
    }

    let bt = target.as_bytes();
    let (n, half) = (bt.len(), bt.len() / 2);
    let mut buf: Vec<u8> = vec![0u8; n];
    if odd == 1 {
      cnt[(ob - b'a') as usize] -= 1;
      buf[half] = ob;
    }

    fn recur(
      bt: &[u8], idx: usize, half: usize, n: usize, cnt: &mut [u16], buf: &mut Vec<u8>,
    ) -> bool {
      if idx == half {
        for i in 0..n {
          if buf[i] != bt[i] {
            return buf[i] > bt[i];
          }
        }
        return false;
      }

      let mirror = n - 1 - idx;
      let offset = (bt[idx] - b'a') as usize;
      if cnt[offset] > 1 {
        cnt[offset] -= 2;
        let candi = b'a' + offset as u8;
        buf[idx] = bt[idx];
        buf[mirror] = bt[idx];
        if recur(bt, idx + 1, half, n, cnt, buf) {
          return true;
        }
        cnt[offset] += 2;
      }

      for i in offset + 1..26 {
        if cnt[i] > 1 {
          cnt[i] -= 2;
          let candi = b'a' + i as u8;
          buf[idx] = candi;
          buf[mirror] = candi;
          let mut nxt: usize = idx + 1;
          for j in 0..26 {
            let candi = b'a' + j as u8;
            let pairs = cnt[j] as usize / 2;
            buf[nxt..nxt + pairs].fill(candi);
            buf[n - nxt - pairs..n - nxt].fill(candi);
            nxt += pairs;
          }
          return true;
        }
      }
      false
    }

    if recur(&bt, 0, half, n, &mut cnt, &mut buf) {
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
