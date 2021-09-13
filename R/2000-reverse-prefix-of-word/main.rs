struct Solution {}

impl Solution {
  pub fn reverse_prefix(word: String, ch: char) -> String {
    let mut bb: Vec<u8> = word.as_bytes().to_vec();
    let mut idx: i32 = -1;
    for i in 0..bb.len() {
      if bb[i] == (ch as u8) {
        idx = i as i32;
        break;
      }
    }
    let mut ret: String;
    if idx == -1 {
      ret = word;
    } else {
      let mut s: i32 = 0;
      let mut e: i32 = idx;

      while s < e {
        let t: u8 = bb[s as usize];
        bb[s as usize] = bb[e as usize];
        bb[e as usize] = t;
        s = s + 1;
        e = e - 1;
      }
      ret = String::from_utf8(bb).unwrap();
    }

    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::reverse_prefix(String::from("cbacdcbc"), 'd')
  );
}
