struct Solution {}

impl Solution {
  pub fn largest_merge(word1: String, word2: String) -> String {
    let mut ret: Vec<u8> = Vec::new();

    let mut idx1: usize = 0;
    let mut idx2: usize = 0;
    let bb1: &str = word1.as_str();
    let bb2: &str = word2.as_str();

    while idx1 < bb1.len() || idx2 < bb2.len() {
      if idx1 < bb1.len() && bb1.get(idx1..).unwrap() > bb2.get(idx2..).unwrap() {
        ret.push(bb1.as_bytes()[idx1]);
        idx1 += 1;
      } else {
        ret.push(bb2.as_bytes()[idx2]);
        idx2 += 1;
      }
    }
    String::from_utf8(ret).unwrap()
  }
}
