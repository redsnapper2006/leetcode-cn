struct Solution {}

impl Solution {
  pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    let s1b: Vec<&str> = sentence1.split(' ').collect();
    let s2b: Vec<&str> = sentence2.split(' ').collect();

    let mut long: Vec<&str> = Vec::new();
    let mut short: Vec<&str> = Vec::new();

    let (mut long, mut short): (Vec<&str>, Vec<&str>) = match s1b.len() < s2b.len() {
      true => (s2b, s1b),
      _ => (s1b, s2b),
    };

    let mut start: usize = 0;
    let mut end: i32 = short.len() as i32 - 1;
    while start < short.len() && short[start] == long[start] {
      start += 1;
    }
    while end >= 0 && short[end as usize] == long[long.len() - short.len() + end as usize] {
      end -= 1;
    }
    start as i32 > end
  }
}
