struct Solution {}

impl Solution {
  pub fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
      return false;
    }

    let mut buf1: Vec<i32> = vec![0; 26];
    let mut buf2: Vec<i32> = vec![0; 26];

    word1
      .as_bytes()
      .iter()
      .for_each(|&c| buf1[(c - b'a') as usize] += 1);
    word2
      .as_bytes()
      .iter()
      .for_each(|&c| buf2[(c - b'a') as usize] += 1);

    let mut idx: usize = 0;
    while idx < 26 {
      if buf1[idx] == 0 && buf2[idx] != 0 || buf1[idx] != 0 && buf2[idx] == 0 {
        return false;
      }
      idx += 1;
    }

    buf1.sort();
    buf2.sort();

    buf1 == buf2
  }

  pub fn close_strings2(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
      return false;
    }

    let mut buf1: Vec<i32> = vec![0; 26];
    let mut buf2: Vec<i32> = vec![0; 26];

    word1
      .as_bytes()
      .iter()
      .for_each(|&c| buf1[(c - b'a') as usize] += 1);
    word2
      .as_bytes()
      .iter()
      .for_each(|&c| buf2[(c - b'a') as usize] += 1);

    // println!("begin {:?} {:?}",buf1, buf2);
    while true {
      let mut offset: usize = 0;
      while offset < 26 {
        if buf1[offset] != 0 {
          break;
        }
        offset += 1;
      }
      if offset == 26 {
        break;
      }
      let mut offset2: usize = 0;
      while offset2 < 26 {
        if buf2[offset2] == buf1[offset] {
          break;
        }
        offset2 += 1;
      }
      if offset2 == 26 {
        return false;
      }
      // println!("offset 1 & 2 {} {} ", offset, offset2);
      if buf1[offset2] == 0 {
        return false;
      }
      buf1[offset] = 0;
      buf2[offset2] = 0;

      buf1[offset] = buf1[offset2];
      buf1[offset2] = 0;
      // println!("in loop {:?} {:?}",buf1, buf2);
    }

    true
  }
}
