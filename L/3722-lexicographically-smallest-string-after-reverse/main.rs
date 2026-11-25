impl Solution {
  pub fn lex_smallest(s: String) -> String {
    let bb = s.as_bytes().to_vec();
    let n = bb.len();
    let mut rev = bb.clone();
    rev.reverse();

    let mut ans: Vec<u8> = vec![b'z'; n];
    for i in 1..=n {
      let mut buf = rev[n - i..].to_vec();
      buf.extend(bb[i..].to_vec());
      if ans > buf {
        ans = buf;
      }

      buf = bb[0..i].to_vec();
      buf.extend(rev[0..n - i].to_vec());
      if ans > buf {
        ans = buf;
      }
    }

    String::from_utf8(ans).unwrap()
  }
}
