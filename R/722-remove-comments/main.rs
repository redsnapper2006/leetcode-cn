impl Solution {
  pub fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut is_block_comment: bool = false;
    let mut buf: Vec<u8> = Vec::new();
    source.iter().for_each(|s| {
      let mut j: usize = 0;
      while j < s.len() {
        if !is_block_comment
          && j + 1 < s.len()
          && s.as_bytes()[j] == b'/'
          && s.as_bytes()[j + 1] == b'*'
        {
          is_block_comment = true;
          j += 2;
          continue;
        }
        if !is_block_comment
          && j + 1 < s.len()
          && s.as_bytes()[j] == b'/'
          && s.as_bytes()[j + 1] == b'/'
        {
          break;
        }
        if is_block_comment
          && j + 1 < s.len()
          && s.as_bytes()[j] == b'*'
          && s.as_bytes()[j + 1] == b'/'
        {
          is_block_comment = false;
          j += 2;
          continue;
        }
        if !is_block_comment {
          buf.push(s.as_bytes()[j]);
        }
        j += 1;
      }
      if buf.len() == 0 {
        return;
      }
      if !is_block_comment {
        res.push(String::from_utf8(buf.clone()).unwrap());
        buf.clear();
      }
    });
    res
  }
}
