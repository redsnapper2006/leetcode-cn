impl Solution {
  pub fn number_of_special_chars(word: String) -> i32 {
    let mut buf: Vec<(bool, bool, bool)> = vec![(false, false, true); 26];
    word.as_bytes().iter().for_each(|&b| match b {
      b'A'..=b'Z' => {
        buf[(b - b'A') as usize].1 = true;
      }
      _ => {
        let offset = (b - b'a') as usize;
        if buf[offset].1 {
          buf[offset].2 = false;
        } else {
          buf[offset].0 = true;
        }
      }
    });

    buf.iter().filter(|&x| x.0 && x.1 && x.2).count() as i32
  }
}
