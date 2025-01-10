struct Solution {}

impl Solution {
  pub fn valid_substring_count(word1: String, word2: String) -> i64 {
    let mut w1: Vec<i32> = vec![0; 26];
    let mut w2: Vec<i32> = vec![0; 26];
    word2.as_bytes().iter().for_each(|&b| {
      w2[(b - b'a') as usize] += 1;
    });

    fn is_match(w1: &Vec<i32>, w2: &Vec<i32>) -> bool {
      let mut idx: usize = 0;
      while idx < 26 {
        if w1[idx] < w2[idx] {
          return false;
        }

        idx += 1;
      }
      true
    }

    let mut start: usize = 0;
    let mut end: usize = 0;
    let bb = word1.as_bytes().to_vec();
    let mut ans: i64 = 0;
    while end < bb.len() {
      w1[(bb[end] - b'a') as usize] += 1;
      while is_match(&w1, &w2) {
        ans += (bb.len() - end) as i64;
        w1[(bb[start] - b'a') as usize] -= 1;
        start += 1;
      }
      end += 1;
    }
    ans
  }
}
