struct Solution {}

impl Solution {
  pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    let mut buf: [i32; 26] = [0; 26];
    let bb = allowed.as_bytes();
    for i in 0..bb.len() {
      let offset = bb[i] - 'a' as u8;
      buf[offset as usize] = 1;
    }

    let mut count: i32 = 0;
    for i in 0..words.len() {
      let mut is_in: bool = true;

      let bb = words[i].as_bytes();
      for j in 0..bb.len() {
        let offset = bb[j] - 'a' as u8;

        if buf[offset as usize] == 0 {
          is_in = false;
          break;
        }
      }
      if is_in {
        count += 1;
      }
    }
    count
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_consistent_strings(
      String::from("ab"),
      vec![
        String::from("ad"),
        String::from("bd"),
        String::from("aaab"),
        String::from("baa"),
        String::from("badab")
      ]
    )
  )
}
