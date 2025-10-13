impl Solution {
  pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    words
      .iter()
      .fold((vec![], vec![0; 26]), |mut ans, word| {
        let aggr = word.as_bytes().iter().fold(vec![0; 26], |mut aggr, b| {
          aggr[(b - b'a') as usize] += 1;
          aggr
        });
        if (ans.1 != aggr) {
          ans.0.push(word.clone());
        }
        (ans.0, aggr)
      })
      .0
  }
}
