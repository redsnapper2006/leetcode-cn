struct Solution {}

impl Solution {
  pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    words.iter().for_each(|w| {
      let mut buf: Vec<u8> = Vec::new();
      for &b in w.as_bytes() {
        if b == separator as u8 {
          if buf.len() != 0 {
            res.push(String::from_utf8(buf).unwrap());
          }
          buf = Vec::new();
        } else {
          buf.push(b);
        }
      }
      if buf.len() != 0 {
        res.push(String::from_utf8(buf).unwrap());
      }
    });
    res
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::split_words_by_separator(
      vec![
        "one.two.three".to_string(),
        "four.five".to_string(),
        "six".to_string()
      ],
      '.'
    )
  )
}
