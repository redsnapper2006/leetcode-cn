struct Solution {}

impl Solution {
  pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    fn replace(a: u8, ak: &Vec<u8>, k: i32) -> i32 {
      let mut max: i32 = 0;
      let mut start: usize = 0;
      let mut end: usize = 0;
      let mut cnt: i32 = 0;
      while end < ak.len() {
        if ak[end] != a {
          cnt += 1;
        }
        if cnt > k {
          while cnt > k && start < ak.len() {
            if ak[start] != a {
              cnt -= 1;
            }
            start += 1;
          }
        }

        end += 1;
        if end - start > max as usize {
          max = (end - start) as i32;
        }
      }
      max
    }
    let buf: Vec<u8> = answer_key.bytes().collect();
    replace('T' as u8, &buf, k).max(replace('F' as u8, &buf, k))
  }
}
