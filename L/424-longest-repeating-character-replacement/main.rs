impl Solution {
  pub fn character_replacement(s: String, k: i32) -> i32 {
    *('A'..='Z')
      .map(|base| {
        let mut start: usize = 0;
        let mut end: usize = 0;

        let mut cnt: i32 = 0;
        while end < s.len() && cnt < k {
          if s.as_bytes()[end] != base as u8 {
            cnt += 1;
          }
          end += 1;
        }

        let mut ret: usize = end - start;
        while end < s.len() {
          if s.as_bytes()[end] == base as u8 {
            end += 1;
            if ret < end - start {
              ret = end - start;
            }
            continue;
          }
          while start <= end {
            if s.as_bytes()[start] != base as u8 {
              break;
            }
            start += 1;
          }
          if ret < end - start {
            ret = end - start;
          }
          start += 1;
          end += 1;
        }
        ret
      })
      .collect::<Vec<usize>>()
      .iter()
      .max()
      .unwrap() as i32
  }
}
