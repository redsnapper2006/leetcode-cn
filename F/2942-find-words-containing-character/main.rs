struct Solution {}

impl Solution {
  pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
    words
      .iter()
      .enumerate()
      .filter(|(idx, word)| word.contains(x))
      .map(|x| x.0 as i32)
      .collect::<Vec<i32>>()
  }

  pub fn find_words_containing2(words: Vec<String>, x: char) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    words.iter().enumerate().for_each(|(i, word)| {
      if word.contains(x) {
        result.push(i as i32);
      }
    });
    result
  }
}
