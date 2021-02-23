struct Solution {}

impl Solution {
  pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut b1: Vec<u8> = word1.as_bytes().to_vec();
    let mut b2: Vec<u8> = word2.as_bytes().to_vec();
    let mut size = b1.len();
    if size < b2.len() {
      size = b2.len();
    }
    b1.resize(size, ' ' as u8);
    b2.resize(size, ' ' as u8);
    let a = b1
      .iter()
      .zip(b2.iter())
      .map(|(x, y)| vec![*x, *y])
      .flatten()
      .filter(|x| *x != ' ' as u8)
      .collect::<Vec<u8>>();
    String::from_utf8(a).unwrap()
  }

  pub fn merge_alternatelyV2(word1: String, word2: String) -> String {
    let b1: Vec<u8> = word1.as_bytes().to_vec();
    let b2: Vec<u8> = word2.as_bytes().to_vec();

    let mut s1 = 0;
    let mut s2 = 0;
    let mut r: Vec<u8> = Vec::new();
    while s1 < b1.len() && s2 < b2.len() {
      r.push(b1[s1]);
      r.push(b2[s2]);
      s1 += 1;
      s2 += 1;
    }
    if s1 < b1.len() {
      for i in s1..b1.len() {
        r.push(b1[i]);
      }
    }
    if s2 < b2.len() {
      for i in s2..b2.len() {
        r.push(b2[i]);
      }
    }
    String::from_utf8(r).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::merge_alternately(String::from("hello"), String::from("world"))
  );
}
