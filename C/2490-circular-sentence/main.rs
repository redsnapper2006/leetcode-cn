struct Solution {}

impl Solution {
  pub fn is_circular_sentence(sentence: String) -> bool {
    let mut words = sentence
      .split(' ')
      .collect::<Vec<&str>>()
      .iter()
      .map(|&v| v.to_string().as_bytes().to_vec())
      .collect::<Vec<Vec<u8>>>();
    words.push(words[0].clone());
    (0..words.len() - 1)
      .map(|idx| words[idx][words[idx].len() - 1] == words[idx + 1][0])
      .all(|x| x)
  }
}
