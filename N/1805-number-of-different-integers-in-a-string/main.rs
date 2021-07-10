struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn num_different_integers(word: String) -> i32 {
    let bb = word.as_bytes();
    let mut sum: Vec<u8> = Vec::new();
    let mut is_num: bool = false;
    let mut m: HashSet<String> = HashSet::new();
    for i in 0..bb.len() {
      if bb[i] >= 'a' as u8 && bb[i] <= 'z' as u8 {
        if is_num {
          let mut idx: usize = 0;
          while idx < sum.len() && sum[idx] == '0' as u8 {
            idx += 1;
          }
          m.insert(String::from_utf8(sum.drain(idx..).collect()).unwrap());
        }
        is_num = false;
        sum = Vec::new();
        continue;
      }
      sum.push(bb[i]);
      is_num = true;
    }
    // println!("{:?} {} {} ", m, isNum, sum);
    if is_num {
      let mut idx: usize = 0;
      while idx < sum.len() && sum[idx] == '0' as u8 {
        idx += 1;
      }
      m.insert(String::from_utf8(sum.drain(idx..).collect()).unwrap());
    }
    m.len() as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::num_different_integers(String::from(
      "035985750011523523129774573439111590559325a1554234973"
    ))
  );
}
