impl Solution {
  pub fn is_acronym(words: Vec<String>, s: String) -> bool {
    String::from_utf8(
      words
        .iter()
        .map(|word| word.as_bytes().to_vec()[0])
        .collect::<Vec<u8>>()
    )
    .unwrap()
      == s
  }
}
