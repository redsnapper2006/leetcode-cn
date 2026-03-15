impl Solution {
  pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
    String::from_utf8(
      words
        .iter()
        .map(|word| {
          b'a'
            + (25
              - word.as_bytes().iter().fold(0, |sum, b| sum + weights[(b - b'a') as usize]) % 26)
              as u8
        })
        .collect::<Vec<u8>>(),
    )
    .unwrap()
  }
}
