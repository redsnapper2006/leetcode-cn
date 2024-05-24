impl Solution {
  pub fn number_of_special_chars(word: String) -> i32 {
    let mut buf: [(i32, i32); 26] = [(0, 0); 26];
    for c in word.chars() {
      if c.is_uppercase() {
        buf[(c as u8 - 'A' as u8) as usize].1 = 1;
      } else {
        buf[(c as u8 - 'a' as u8) as usize].0 = 1;
      }
    }
    buf.iter().filter(|x| x.0 == 1 && x.1 == 1).count() as i32
  }
}
