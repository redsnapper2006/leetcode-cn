struct Solution {}

impl Solution {
  pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let mut idxs: Vec<Vec<usize>> = Vec::new();
    for _i in 0..26 {
      idxs.push(Vec::new());
    }
    for (i, b) in s.as_bytes().iter().enumerate() {
      let offset = *b as u8 - 'a' as u8;
      idxs[offset as usize].push(i);
    }
    let mut ret: i32 = 0;
    for word in words {
      let mut idx: i32 = -1;
      let mut is_f: bool = true;
      for b in word.as_bytes() {
        let offset: usize = (*b as u8 - 'a' as u8) as usize;
        let p = idxs[offset].partition_point(|&x| x < (idx + 1) as usize);
        if p == idxs[offset].len() {
          is_f = false;
          break;
        }
        idx = idxs[offset][p] as i32;
      }

      if is_f {
        ret += 1;
      }
    }

    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::num_matching_subseq("abacdasfasdfasdfaf".to_string(), vec!["a".to_string()])
  )
}
