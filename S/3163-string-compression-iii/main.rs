impl Solution {
  pub fn compressed_string(word: String) -> String {
    let mut ans: Vec<u8> = Vec::new();

    let mut cnt: i32 = 0;
    let mut base: u8 = b'0';
    word.as_bytes().iter().for_each(|&b| {
      if base != b {
        if cnt != 0 {
          ans.push((b'0' + cnt as u8));
          ans.push(base);
        }
        cnt = 1;
        base = b;
      } else {
        cnt += 1;
        if cnt == 10 {
          ans.push(b'9');
          ans.push(base);
          cnt = 1;
        }
      }
    });
    ans.push((b'0' + cnt as u8));
    ans.push(base);
    String::from_utf8(ans).unwrap()
  }
}
